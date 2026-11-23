# mvn dependency

Manage and analyze Maven project dependencies. Provides goals for viewing, resolving, and copying project dependencies.

## Usage
- `mvn dependency:add -Dgav=group_id:artifact_id:version` : add a dependency
- `mvn dependency:tree` : display the full dependency tree, including direct and transitive dependencies
- `mvn dependency:analyze` : analyze the dependencies and highlight unused or undeclared ones
- `mvn dependency:copy-dependencies` : copy all project dependencies (by default, to `target/dependency/`)
- `mvn dependency:resolve` : resolve and download all project dependencies to the local Maven repository
- `mvn dependency:resolve -U` : force Maven to update all dependencies from remote repositories

## Categories
packages, development, network

## Links
- More information: https://maven.apache.org/plugins/maven-dependency-plugin/usage.html
- tldr page: https://tldr.inbrowser.app/pages/common/mvn-dependency
