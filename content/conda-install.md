# conda install

Install packages into an existing conda environment.

## Usage
- `conda install package1 package2 ...` : install one or more package into the currently active conda environment
- `conda install -c conda-forge package` : install a single package into the currently active conda environment using channel conda-forge
- `conda install -c conda-forge --override-channels package` : install a single package into the currently active conda environment using channel conda-forge and ignoring other channels
- `conda install package=version` : install a specific version of a package
- `conda install -n environment package` : install a package into a specific environment
- `conda install --upgrade package` : update a package in the current environment
- `conda install -y package` : install a package and agree to the transactions without prompting

## Categories
packages

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/install.html
- tldr page: https://tldr.inbrowser.app/pages/common/conda-install
