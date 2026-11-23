# az quantum

Manage Azure Quantum workspaces and submit quantum jobs to providers (preview, requires quantum extension).

## Usage
- `az quantum workspace create -g ResourceGroup -l Location -w Workspace -a MyStorageAccountName` : create a new Azure Quantum workspace
- `az quantum workspace list` : list all Azure Quantum workspaces
- `az quantum workspace set -g ResourceGroup -w Workspace` : set a default Azure Quantum workspace
- `az quantum job submit -g ResourceGroup -w Workspace -l Location -t target_id --job-name Job --job-input-file QirBitcode.bc --job-input-format qir.v1` : submit a QIR quantum job to a target
- `az quantum job list -g ResourceGroup -l Location -w Workspace` : list all jobs in a Quantum Workspace
- `az quantum job output -g ResourceGroup -w Workspace --job-id Job` : get the output of a quantum job
- `az quantum offerings list -l Location` : list available provider offerings in a location
- `az quantum target set -t target_id` : set a default target for job submissions

## Categories
processes, cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/quantum
- tldr page: https://tldr.inbrowser.app/pages/common/az-quantum
