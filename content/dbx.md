# dbx

Interact with the Databricks platform. Note: This tool has been retired and it is recommended to use Databricks Asset Bundles instead.

## Usage
- `dbx configure --profile DEFAULT` : create a new `dbx` project in the current working directory
- `dbx sync dbfs --source path/to/directory --dest path/to/remote_directory` : sync local files from the specified path to DBFS and watch for changes
- `dbx deploy workflow_name` : deploy the specified workflow to artifact storage
- `dbx launch workflow_name` : launch the specified workflow after deploying it

## Categories
files

## Links
- More information: https://dbx.readthedocs.io/en/latest/reference/cli/#dbx
- tldr page: https://tldr.inbrowser.app/pages/common/dbx
