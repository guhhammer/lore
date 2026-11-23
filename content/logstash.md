# logstash

An Elasticsearch ETL (extract, transform, and load) tool. Commonly used to load data from various sources (such as databases and log files) into Elasticsearch.

## Usage
- `logstash --configtest --config logstash_config.conf` : check validity of a Logstash configuration
- `sudo logstash --config logstash_config.conf` : run Logstash using configuration
- `sudo logstash -e 'input {} filter {} output {}'` : run Logstash with the most basic inline configuration string

## Categories
monitoring, archives

## Links
- More information: https://www.elastic.co/logstash
- tldr page: https://tldr.inbrowser.app/pages/common/logstash
