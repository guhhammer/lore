# mvn site

Generate a project website based on the information in the `pom.xml` file.

## Usage
- `mvn site` : generate a project site using the site plugin
- `mvn site -pl module_name` : generate a site for a specific Maven project (multi-module build)
- `mvn clean site` : clean previous site output before generating a new one
- `mvn site -D skipTests` : skip tests while generating the site
- `mvn site-deploy` : generate and deploy the site to the remote server

## Categories
development, files, text

## Links
- More information: https://maven.apache.org/plugins/maven-site-plugin/usage.html
- tldr page: https://tldr.inbrowser.app/pages/common/mvn-site
