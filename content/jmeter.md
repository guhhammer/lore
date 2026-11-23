# jmeter

Open source Java application designed for load testing functional behavior and measure performance.

## Usage
- `jmeter -n -t path/to/file.jmx` : run a specific test plan in nongui mode
- `jmeter -n -t path/to/file.jmx -l path/to/logfile.jtl` : run a test plan in nongui mode using a specific log file
- `jmeter -n -t path/to/file.jmx -H- 127.0.0.1 -P 8888` : run a test plan in nongui mode using a specific proxy
- `jmeter -n -t path/to/file.jmx -J key='value'` : run a test plan in nongui mode using a specific JMeter property

## Categories
files, monitoring, development

## Links
- More information: https://jmeter.apache.org/usermanual/get-started.html#options
- tldr page: https://tldr.inbrowser.app/pages/common/jmeter
