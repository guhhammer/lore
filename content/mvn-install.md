# mvn install

Install third-party Maven dependencies and build the project.

## Usage
- `mvn install` : compile, test, package, and install the project into the local repository
- `mvn install -D skipTests` : skip tests during installation
- `mvn install -U` : force update of dependencies before installing
- `mvn install -D maven.test.skip=true` : skip test compilation and execution

## Categories
packages, development

## Links
- More information: https://manned.org/mvn
- tldr page: https://tldr.inbrowser.app/pages/common/mvn-install
