# speedtest

Official command-line interface for testing internet bandwidth using <https://speedtest.net>. These command examples apply only to the official client. Note: Some platforms link `speedtest` to `speedtest-cli` or other tools like `librespeed`, which can also be installed as `speedtest` on certain Linux distributions.

## Usage
- `speedtest` : run a speed test
- `speedtest -u auto-decimal-bits|auto-decimal-bytes|auto-binary-bits|auto-binary-bytes` : run a speed test and specify the unit of the output
- `speedtest -f human-readable|csv|tsv|json|jsonl|json-pretty` : run a speed test and specify the output format
- `speedtest -P 0..8` : run a speed test and specify the number of decimal points to use (0 to 8, defaults to 2)
- `speedtest -p yes|no` : run a speed test and print its progress (only available for output format `human-readable` and `json`)
- `speedtest -L` : list all `speedtest.net` servers, sorted by distance
- `speedtest -s server_id` : run a speed test to a specific `speedtest.net` server

## Categories
network, text, development

## Links
- More information: https://www.speedtest.net/apps/cli
- tldr page: https://tldr.inbrowser.app/pages/common/speedtest
