# crackle

Crack and decrypt Bluetooth Low Energy (BLE) encryption.

## Usage
- `crackle -i path/to/input.pcap` : check whether the recorded BLE communications contain the packets necessary for recovering temporary keys (TKs)
- `crackle -i path/to/input.pcap -o path/to/decrypted.pcap` : use brute force to recover the TK of the recorded pairing events and use it to decrypt all subsequent communications
- `crackle -i path/to/input.pcap -o path/to/decrypted.pcap -l 81b06facd90fe7a6e9bbd9cee59736a7` : use the specified long-term key (LTK) to decrypt the recorded communication

## Categories
files, security, hardware

## Links
- More information: https://github.com/mikeryan/crackle
- tldr page: https://tldr.inbrowser.app/pages/common/crackle
