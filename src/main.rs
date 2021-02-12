extern crate clap;
use clap::{App, Arg};
//use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define arguments, flags, and help information for command
    // This is all done through the clap crate
    let matches = App::new("Weather Forecast display from Open Weather API")
        .version("1.0")
        .author("Anooj Patel <anooj.r.patel@gmail.com>")
        .about("View Weather Forecast")
        .arg(
            Arg::with_name("CITY")
                .help("The city whose weather you want")
                .index(1),
        )
        .arg(
            Arg::with_name("COUNTRY_CODE")
                .help("The 2 character abbreviation of the country that your city is in (optional)")
                .index(2),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("m")
                .short("m")
                .long("m")
                .help("Sets units to metic"),
        )
        .arg(
            Arg::with_name("s")
                .short("s")
                .long("s")
                .help("Sets units to standard")
                // Flags -m and -s cannot both be set!
                .conflicts_with("m"),
        )
        .get_matches();

    // Set API key, grab cit an country (if it exists, else default value), units default to imperial
    let app_id: String = String::from("***REMOVED***");
    let query;
    let (city, country) = (
        matches.value_of("CITY").unwrap_or("No city selected"),
        matches
            .value_of("COUNTRY_CODE")
            .unwrap_or("No country selected"),
    );
    let mut units: String = String::from("imperial");
    let mut temp: String = String::from("\u{00B0}F");
    let mut speed: String = String::from("miles per hour");
    let resp: serde_json::Value;

    // Check for unit flags and if so change units
    if matches.is_present("s") {
        units = String::from("standard");
        temp = String::from("K");
        speed = String::from("meters per second");
    } else if matches.is_present("m") {
        units = String::from("metric");
        temp = String::from("\u{00B0}C");
        speed = String::from("meters per second");
    }

    // Construct query based on whether a country was supplied or not
    if city == "No city selected" {
        // If no location is provided, then a call is made to an IP geolocation API to determine city and country
        let loc_query = "http://ip-api.com/json/";

        let loc_json: serde_json::Value =
            serde_json::from_str(&reqwest::get(loc_query).await?.text().await?).unwrap();

        assert_eq!(
            loc_json.pointer("/status").unwrap(),
            "success",
            "{}",
            loc_json.to_string()
        );

        query = format! {"https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units={}",
        loc_json.pointer("/city").unwrap().as_str().unwrap(),
        loc_json.pointer("/countryCode").unwrap().as_str().unwrap(),
        app_id,
        units};

        resp = serde_json::from_str(&reqwest::get(&query).await?.text().await?).unwrap();
    } else {
        if country == "No country selected" {
            query = format! {"https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}", city, app_id, units};
        } else {
            query = format! {"https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units={}", city, country, app_id, units};
        }

        resp = serde_json::from_str(&reqwest::get(&query).await?.text().await?).unwrap();
    }

    /*
    Parse the returned JSON into a serde_json::Value object
    This is this most convoluted part, the steps are as follows:
    1. Send the query and await the response
    2. Convert it to text and await that return
    3. Create the serde_json::Value from that text and then unwrap the Option object that is returned
    We always expect a return from the query because the API will return a JSON containing any errors
    */
    // If no error proceed with printing out information
    if resp.pointer("/cod").unwrap() != 404 {
        // Vary the output based on how many times the user used the "verbose" flag
        // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
        // The v flag functionality is from the clap crate
        match matches.occurrences_of("v") {
        0 => println!(
            "City: {}\nCountry: {}\nTemperature: {}{}",
            resp.pointer("/name").unwrap().as_str().unwrap(),
            resp.pointer("/sys/country").unwrap().as_str().unwrap(),
            resp.pointer("/main/temp").unwrap(),
            temp
        ),
        1 => println!(
            "City: {}\nCountry: {}\nTemperature: {}{}\nDescription: {}",
            resp.pointer("/name").unwrap().as_str().unwrap(),
            resp.pointer("/sys/country").unwrap().as_str().unwrap(),
            resp.pointer("/main/temp").unwrap(),
            temp,
            resp.pointer("/weather/0/description").unwrap(),
        ),
        2 => println!(
            "City: {}\nCountry: {}\nTemperature: {}{}\nDescription: {}\nHumidity: {}%\nWind Speed: {} {}",
            resp.pointer("/name").unwrap().as_str().unwrap(),
            resp.pointer("/sys/country").unwrap().as_str().unwrap(),
            resp.pointer("/main/temp").unwrap(),
            temp,
            resp.pointer("/weather/0/description").unwrap(),
            resp.pointer("/main/humidity").unwrap(),
            resp.pointer("/wind/speed").unwrap(),
            speed
        ),
        3 | _ => println!(
            "City: {}\nCountry: {}\nTemperature: {}{}\nFeels Like: {}{}\nDescription: {}\nHumidity: {}%\nWind Speed: {} {}\nTemperature Low: {}{}\nTemperature High: {}{}",
            resp.pointer("/name").unwrap().as_str().unwrap(),
            resp.pointer("/sys/country").unwrap().as_str().unwrap(),
            resp.pointer("/main/temp").unwrap(),
            temp,
            resp.pointer("/main/feels_like").unwrap(),
            temp,
            resp.pointer("/weather/0/description").unwrap(),
            resp.pointer("/main/humidity").unwrap(),
            resp.pointer("/wind/speed").unwrap(),
            speed,
            resp.pointer("/main/temp_min").unwrap(),
            temp,
            resp.pointer("/main/temp_max").unwrap(),
            temp,
        ),
    }
    } else {
        // Else if error print simple message
        println!("Error city not found!");
    }

    Ok(())
}
