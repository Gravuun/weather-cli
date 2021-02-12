# Weather forecast CLI

## Description

Posts a call to OpenWeatherMaps API and displays data based on selected
verbosity and units. If no location is specified a call is posted to a
geolocation API.

### clap generated help documentation

View Weather Forecast

USAGE:
fcast [FLAGS] [CITY] [COUNTRY_CODE]

FLAGS:
-h, --help Prints help information
-m, --m Sets units to metic
-s, --s Sets units to standard
-v Sets the level of verbosity
-V, --version Prints version information

ARGS:
[CITY] The city whose weather you want
[COUNTRY] The 2 character abbreviation of the country that your city is in (optional)

### Libraries

clap
serde
serde_json
tokio
reqwest
