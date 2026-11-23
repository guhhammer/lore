# sc_warts2pcap

Write packets included in `.warts` files to a PCAP file. This is only possible for packets of type `tbit`, `sting`, or `sniff`.

## Usage
- `sc_warts2pcap -o path/to/output.pcap path/to/file1.warts path/to/file2.warts ...` : convert the data from several `.warts` files into one PCAP file
- `sc_warts2pcap -s -o path/to/output.pcap path/to/file.warts` : convert the data from a `.warts` file into a PCAP file and sort the packets by timestamp

## Categories
files

## Links
- More information: https://www.caida.org/catalog/software/scamper/
- tldr page: https://tldr.inbrowser.app/pages/common/sc_warts2pcap
