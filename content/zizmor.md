# zizmor

Find security issues in GitHub Actions workflows and action definitions.

## Usage
- `zizmor .` : audit all workflows and actions in the current directory
- `zizmor path/to/workflow.yml` : audit a specific workflow file
- `zizmor --gh-token token username/repository` : audit a remote GitHub repository (requires a GitHub token)
- `zizmor -o path/to/workflow.yml` : run only offline audits (no network requests)
- `zizmor --format sarif path/to/workflow.yml` : output results in SARIF format
- `zizmor --min-severity informational|low|medium|high .` : filter findings by minimum severity
- `zizmor -h` : display help
- `zizmor -v` : display version

## Categories
files, network

## Links
- More information: https://docs.zizmor.sh/quickstart/
- tldr page: https://tldr.inbrowser.app/pages/common/zizmor
