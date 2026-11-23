# dbt

A tool to model transformations in data warehouses.

## Usage
- `dbt debug` : debug the dbt project and the connection to the database
- `dbt run` : run all models of the project
- `dbt test --select example_model` : run all tests of `example_model`
- `dbt build --select example_model+` : build (load seeds, run models, snapshots, and tests associated with) `example_model` and its downstream dependents
- `dbt build --exclude "tag:not_now"` : build all models, except the ones with the tag `not_now`
- `dbt build --select "tag:one,tag:two"` : build all models with tags `one` and `two`
- `dbt build --select "tag:one tag:two"` : build all models with tags `one` or `two`

## Categories
development

## Links
- More information: https://github.com/dbt-labs/dbt-core
- tldr page: https://tldr.inbrowser.app/pages/common/dbt
