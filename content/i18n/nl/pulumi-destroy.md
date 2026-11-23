# pulumi destroy

Vernietig alle bestaande bronnen in een stack.

## Usage
- `pulumi destroy` : vernietig alle bronnen in de huidige stack
- `pulumi destroy -s stack` : vernietig alle bronnen in een specifieke stack
- `pulumi destroy -y` : keur automatisch bronnen goed en vernietig deze na voorvertoning
- `pulumi destroy --exclude-protected` : sluit beschermde bronnen uit van vernietiging
- `pulumi destroy --remove` : verwijder de stack en het bijbehorende configuratiebestand nadat alle bronnen in de stack zijn verwijderd
- `pulumi destroy --continue-on-error` : ga door met de bronnen vernietigen, zelfs als er een fout optreedt

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_destroy/
