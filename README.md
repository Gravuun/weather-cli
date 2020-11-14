# Weather forecast CLI

## Description

Posts a call to OpenWeatherMaps API and displays data based on selected
verbosity and units.

### clap generated help documentation

View Weather Forecast

USAGE:
fcast [FLAGS] <CITY> [COUNTRY]

FLAGS:
-h, --help Prints help information
-m, --m Sets units to metic
-s, --s Sets units to standard
-v Sets the level of verbosity
-V, --version Prints version information

ARGS:
<CITY> The city whose weather you want
<COUNTRY> The 2 character abbreviation of the country that your city is in (optional)

### Libraries

clap
serde
serde_json
tokio
reqwest
