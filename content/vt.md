# vt

Interface for VirusTotal. API key from a VirusTotal account is required for this command.

## Usage
- `vt scan file path/to/file` : scan a specific file for viruses
- `vt scan url url` : scan a URL for viruses
- `vt analysis file_id|analysis_id` : display information from a specific analysis
- `vt download file_id --output path/to/directory --zip --zip-password password` : download files in encrypted Zip format (requires premium account)
- `vt init` : initialize or re-initialize `vt` to enter API key interactively
- `vt domain url` : display information about a domain
- `vt url url` : display information for a specific URL
- `vt domain ip_address` : display information for a specific IP address

## Categories
network, files, users

## Links
- More information: https://github.com/VirusTotal/vt-cli
- tldr page: https://tldr.inbrowser.app/pages/common/vt
