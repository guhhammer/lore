# mvn generate-sources

Generate source code for a Maven project before the main compilation phase. This phase runs after `initialize` and before `process-sources`.

## Usage
- `mvn generate-sources` : run all lifecycle phases up to `generate-sources`
- `mvn generate-resources` : run the next phase to generate resources
- `mvn clean generate-sources` : clean and regenerate sources

## Categories
development, processes

## Links
- More information: https://manned.org/mvn
- tldr page: https://tldr.inbrowser.app/pages/common/mvn-generate-sources
