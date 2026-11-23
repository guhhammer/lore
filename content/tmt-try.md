# tmt try

Quickly experiment with tests and environments.

## Usage
- `tmt try` : quickly experiment with the default provision method (no tests in the CWD)
- `cd path/to/test_directory && tmt try` : run a test in the current working directory
- `tmt try rhel-9` : use a specific operating system
- `tmt try fedora@container` : select both custom image and provision method
- `tmt try -t feature` : select tests with custom filter
- `tmt try -a` : provision guest and wait for instructions
- `tmt try -l` : directly log into the guest without asking
- `tmt try --help` : display help

## Categories
development

## Links
- More information: https://tmt.readthedocs.io/en/stable/stories/cli.html#try
- tldr page: https://tldr.inbrowser.app/pages/linux/tmt-try
