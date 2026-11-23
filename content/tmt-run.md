# tmt run

Execute `tmt` test steps. By default, all steps are run.

## Usage
- `tmt run` : run all test steps for each plan
- `tmt run discover -v` : run only the discover step to show what tests would be run
- `tmt run -a provision -h container -i fedora:rawhide` : run all steps and adjust the provision step options
- `tmt run plan -n /plan/name test -n /test/name` : run only selected plans and tests
- `tmt run -l report -h html -o` : show results from the last run in a web browser
- `tmt run -c key=value -c distro=fedora` : run tests with the provided context
- `tmt run -a execute -h tmt --interactive` : run tests interactively (debug test code in the middle of a test)
- `tmt run -n -vvv` : simulate a run to see what actions would happen with the highest verbosity

## Categories
development

## Links
- More information: https://tmt.readthedocs.io/en/stable/stories/cli.html#run
- tldr page: https://tldr.inbrowser.app/pages/linux/tmt-run
