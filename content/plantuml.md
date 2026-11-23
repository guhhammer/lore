# plantuml

Create UML diagrams from a plain text language and render them in different formats.

## Usage
- `plantuml diagram1.puml diagram2.puml` : render diagrams to default format (PNG)
- `plantuml -f format diagram.puml` : render a diagram in given format (e.g. `png`, `pdf`, `svg`, `txt`)
- `plantuml path/to/diagrams` : render all diagrams of a directory
- `plantuml -o path/to/output diagram.puml` : render a diagram to the output directory
- `plantuml -nometadata diagram.png >diagram.puml` : render a diagram without storing the diagram's source code (Note: It's stored by default when the `-nometadata` option isn't specified)
- `plantuml -metadata diagram.png >diagram.puml` : retrieve source from a `plantuml` diagram's metadata
- `plantuml --config config.cfg diagram.puml` : render a diagram with the configuration file
- `plantuml -h` : display help

## Categories
files, text

## Links
- More information: https://plantuml.com/en/command-line
- tldr page: https://tldr.inbrowser.app/pages/common/plantuml
