# tmt

Test Management Tool for creating, running, and debugging tests. Some subcommands such as `run`, `try`, etc. have their own usage documentation.

## Usage
- `tmt` : list available tests, plans, and stories
- `tmt init` : initialize tmt files/project structure
- `tmt test create -t beakerlib --link verifies:issue#1234` : create a new test with a template and a link
- `tmt test|plan|story ls pattern` : list available tests, plans, or stories
- `tmt -c arch=aarch64 test show` : show detailed test metadata in the given context
- `tmt lint` : validate tmt files against the specification
- `tmt tests ls -f tag:foo -f tier:0` : use filter
- `tmt --help` : display help

## Categories
development, documents, monitoring

## Links
- More information: https://tmt.readthedocs.io/en/stable/examples.html
- tldr page: https://tldr.inbrowser.app/pages/linux/tmt
