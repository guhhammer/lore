# ooniprobe

Open Observatory of Network Interference (OONI). Test the blocking of websites and apps. Measure the speed and performance of your network.

## Usage
- `ooniprobe list` : list all tests performed
- `ooniprobe list 7` : show information about a specific test
- `ooniprobe run all` : run all available tests
- `ooniprobe run performance` : perform a specific test
- `ooniprobe run websites --input https://ooni.org/` : check the availability of a specific website
- `ooniprobe run websites --input-file path/to/my-websites.txt` : check the availability of all websites listed in a file
- `ooniprobe show 9` : display detailed information about a test in JSON format

## Categories
network, development, monitoring

## Links
- More information: https://ooni.org/support/ooni-probe-cli/
- tldr page: https://tldr.inbrowser.app/pages/common/ooniprobe
