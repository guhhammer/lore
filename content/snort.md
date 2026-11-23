# snort

Open-source network intrusion detection system.

## Usage
- `sudo snort -v -i interface` : capture packets with verbose output
- `sudo snort -vd -i interface` : capture packets and dump application layer data with verbose output
- `sudo snort -ve -i interface` : capture packets and display link layer packet headers with verbose output
- `sudo snort -i interface -l path/to/directory` : capture packets and save them in the specified directory
- `sudo snort -i interface -c path/to/rules.conf -l path/to/directory` : capture packets according to rules and save offending packets along with alerts

## Categories
network, files, users

## Links
- More information: https://www.snort.org/#documents
- tldr page: https://tldr.inbrowser.app/pages/common/snort
