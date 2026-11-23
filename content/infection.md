# infection

A mutation testing framework for PHP.

## Usage
- `infection` : analyze code using the configuration file (or create one if it does not exist)
- `infection -j number_of_threads` : use a specific number of threads
- `infection --min-msi percentage` : specify a minimum Mutation Score Indicator (MSI)
- `infection --min-covered-msi percentage` : specify a minimum covered code MSI
- `infection --test-framework phpunit|phpspec` : use a specific test framework (defaults to PHPUnit)
- `infection --only-covered` : only mutate lines of code that are covered by tests
- `infection -s` : display the mutation code that has been applied
- `infection --log-verbosity default|all|none` : specify the log verbosity

## Categories
development

## Links
- More information: https://infection.github.io/guide/command-line-options.html
- tldr page: https://tldr.inbrowser.app/pages/common/infection
