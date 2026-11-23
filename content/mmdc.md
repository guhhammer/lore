# mmdc

CLI for mermaid, a diagram generation tool with a domain-specific language. A mermaid definition file is taken as input and an SVG, PNG, or PDF file is generated as output.

## Usage
- `mmdc -i input.mmd -o output.svg` : convert a file to the specified format (automatically determined from the file extension)
- `mmdc -i input.mmd -o output.svg -t forest|dark|neutral|default` : specify the theme of the chart
- `mmdc -i input.mmd -o output.svg -b color` : specify the background color of the chart (e.g. `lime`, `"#D8064F"`, or `transparent`)

## Categories
files, media, documents

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/mmdc
