extern crate clap;
use clap::{App, Arg};

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
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("COUNTRY")
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
    let app_id: String = String::from("936072078a6adb16ab7e39e3c889ebe0");
    let query;
    let (city, country) = (
        matches.value_of("CITY").unwrap(),
        matches.value_of("COUNTRY").unwrap_or("No country selected"),
    );
    let mut units: String = String::from("imperial");
    let mut temp: String = String::from("\u{2109}");
    let mut speed: String = String::from("miles per hour");

    // Check for unit flags and if so change units
    if matches.is_present("s") {
        units = String::from("standard");
        temp = String::from("K");
        speed = String::from("meters per second");
    } else if matches.is_present("m") {
        units = String::from("metric");
        temp = String::from("\u{2103}");
        speed = String::from("meters per second");
    }

    // Construct query based on whether a country was supplied or not
    if country == "No country selected" {
        query = format! {"https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}", city, app_id, units};
    } else {
        query = format! {"https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units={}", city, country, app_id, units};
    }

    /*
    Parse the returned JSON into a serde_json::Value object
    This is this most convoluted part, the steps are as follows:
    1. Send the query and await the response
    2. Convert it to text and await that return
    3. Create the serde_json::Value from that text and then unwrap the Option object that is returned
    We always expect a return from the query because the API will return a JSON containing any errors
    */
    let resp: serde_json::Value =
        serde_json::from_str(&reqwest::get(&query).await?.text().await?).unwrap();

    // If no error proceed with printing out information
    if resp.pointer("/cod").unwrap() != 404 {
        // Vary the output based on how many times the user used the "verbose" flag
        // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
        // The v flag functionality is from the clap crate
        match matches.occurrences_of("v") {
        0 => println!(
            "City: {}\nTemperature: {}{}",
            city,
            resp.pointer("/main/temp").unwrap(),
            temp
        ),
        1 => println!(
            "City: {}\nTemperature: {}{}\nDescription: {}",
            city,
            resp.pointer("/main/temp").unwrap(),
            temp,
            resp.pointer("/weather/0/description").unwrap(),
        ),
        2 => println!(
            "City: {}\nTemperature: {}{}\nDescription: {}\nHumidity: {}%\nWind Speed: {} {}",
            city,
            resp.pointer("/main/temp").unwrap(),
            temp,
            resp.pointer("/weather/0/description").unwrap(),
            resp.pointer("/main/humidity").unwrap(),
            resp.pointer("/wind/speed").unwrap(),
            speed
        ),
        3 | _ => println!(
            "City: {}\nTemperature: {}{}\nFeels Like: {}{}\nDescription: {}\nHumidity: {}%\nWind Speed: {} {}\nTemperature Low: {}{}\nTemperature High: {}{}",
            city,
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
