# tofu output

Export structured data about your OpenTofu resources.

## Usage
- `tofu output` : display all outputs for the root module
- `tofu output name` : output only a value with specific name
- `tofu output -raw` : convert the output value to a raw string (useful for shell scripts)
- `tofu output -json` : format the outputs as a JSON object, with a key per output (useful with `jq`)

## Categories
text

## Links
- More information: https://opentofu.org/docs/cli/commands/output/
- tldr page: https://tldr.inbrowser.app/pages/common/tofu-output
