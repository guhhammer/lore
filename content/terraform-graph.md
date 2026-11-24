# terraform graph

Generate a visual representation of configuration or execution plan. The output is in DOT format and can be rendered using GraphViz.

## Usage
- `terraform graph` : generate a dependency graph for the current configuration
- `terraform graph | dot -Tpng >path/to/graph.png` : generate a graph and render it to a PNG file using GraphViz
- `terraform graph -plan=path/to/tfplan` : generate a graph from a saved plan file
- `terraform graph -draw-cycles` : generate a graph highlighting dependency cycles
- `terraform graph -type=plan|plan-refresh-only|plan-destroy|apply` : generate a graph of a specific type (plan, apply, etc.)

## Categories
files

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/graph
- tldr page: https://tldr.inbrowser.app/pages/common/terraform-graph
- Wikipedia: https://en.wikipedia.org/wiki/Terraform_(software)
