# bats

Bash Automated Testing System: a TAP (<https://testanything.org/>) compliant testing framework for Bash.

## Usage
- `bats -t path/to/test.bats` : run a BATS test script and output results in the TAP (Test Anything Protocol) format
- `bats -c path/to/test.bats` : count test cases of a test script without running any tests
- `bats -r path/to/directory` : run BATS test cases recursively (files with a `.bats` extension)
- `bats -F pretty|tap|tap13|junit path/to/test.bats` : output results in a specific format
- `bats -T path/to/test.bats` : add timing information to tests
- `bats -j number path/to/test.bats` : run specific number of jobs in parallel (requires GNU `parallel` to be installed)

## Categories
development, files, network

## Links
- More information: https://bats-core.readthedocs.io/en/stable/usage.html
- tldr page: https://tldr.inbrowser.app/pages/common/bats
