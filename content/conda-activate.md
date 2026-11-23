# conda activate

Activate a conda environment.

## Usage
- `conda activate myenv` : activate an existing environment named `myenv`
- `conda activate path/to/myenv` : activate an existing environment located at custom path
- `conda activate --stack myenv` : stack `myenv` environment on top of a previous environment making libraries/commands/variables from both accessible
- `conda activate --no-stack myenv` : start a clean environment `myenv` without stacking it making previous environment libraries/commands/variables not accessible
- `conda activate -h` : display help

## See also
- conda deactivate

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/conda-activate
