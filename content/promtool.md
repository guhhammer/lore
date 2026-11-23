# promtool

Tooling for the Prometheus monitoring system.

## Usage
- `promtool check config config_file.yml` : check if the configuration files are valid or not (if present report errors)
- `promtool check rules rules_file.yml` : check if the rule files are valid or not (if present report errors)
- `curl --silent http://example.com:9090/metrics/ | promtool check metrics` : pass Prometheus metrics over `stdin` to check them for consistency and correctness
- `promtool test rules test_file.yml` : run unit tests for rules config

## Categories
network

## Links
- More information: https://prometheus.io/docs/prometheus/latest/getting_started/
- tldr page: https://tldr.inbrowser.app/pages/common/promtool
