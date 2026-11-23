# influx

InfluxDB v1 client.

## Usage
- `influx` : connect to an InfluxDB running on localhost with no credentials
- `influx -username username -password ""` : connect with a specific username (will prompt for a password)
- `influx -host hostname` : connect to a specific host
- `influx -database database_name` : use a specific database
- `influx -execute "influxql_command"` : execute a given command
- `influx -execute "influxql_command" -format json|csv|column` : return output in a specific format

## Categories
network, text

## Links
- More information: https://docs.influxdata.com/influxdb/v1/tools/influx-cli/use-influx-cli/
- tldr page: https://tldr.inbrowser.app/pages/common/influx
