# terraform

Terraform describes infrastructure in files and makes the cloud match them. The loop is init, plan, apply. OpenTofu uses the same commands under the name tofu.

## Tasks
### The usual loop
- `terraform init` : download providers and set up the folder
- `terraform fmt` : format the files
- `terraform validate` : check the configuration is valid
- `terraform plan` : show what would change
- `terraform plan -out=plan.tfplan` : save the plan
- `terraform apply` : make the changes, after a yes
- `terraform apply plan.tfplan` : apply exactly the saved plan
- `terraform destroy` : delete everything this configuration manages (deletes resources)
### Look at the state
- `terraform show` : the current state in readable form
- `terraform state list` : resources in the state
- `terraform state show ADDRESS` : one resource's details
- `terraform output` : the outputs
- `terraform output -raw NAME` : one output, bare, for scripts
### Change the state
- `terraform import ADDRESS ID` : bring an existing resource under Terraform
- `terraform state mv OLD NEW` : rename a resource without recreating it
- `terraform state rm ADDRESS` : forget a resource without deleting it
- `terraform taint ADDRESS` : mark a resource to be recreated (or `apply -replace=ADDRESS`)
- `terraform force-unlock LOCK_ID` : free a stuck state lock
### Variables and targets
- `terraform apply -var 'name=value'` : set a variable
- `terraform apply -var-file=prod.tfvars` : set variables from a file
- `terraform apply -target=ADDRESS` : change only one resource
- `terraform plan -refresh-only` : compare the state with reality
### Workspaces and modules
- `terraform workspace list` : separate states in one folder
- `terraform workspace new NAME` : make a workspace
- `terraform get` : download modules
- `terraform providers` : the providers in use
- `terraform init -upgrade` : move to newer provider versions

## Notes
- Always read the plan: lines starting with `-/+` mean a resource is destroyed and recreated.
- The state file holds secrets and the truth: keep it in a remote backend with locking, not in git.
- `destroy` and `state rm` are not undoable: plan first with `plan -destroy`.
- Commit `.terraform.lock.hcl`; never commit `.terraform/` or `*.tfstate`.
- Install from the official page in Links.

## See also
- ansible
- aws
- gcloud
- az

## Links
- CLI docs: https://developer.hashicorp.com/terraform/cli
- Install: https://developer.hashicorp.com/terraform/install

## Categories
cloud
