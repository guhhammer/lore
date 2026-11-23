# kdesrc-build

Easily build KDE components from its source repositories.

## Usage
- `kdesrc-build --initial-setup` : initialize `kdesrc-build`
- `kdesrc-build component_name` : compile a KDE component and its dependencies from source
- `kdesrc-build --no-src --no-include-dependencies component_name` : compile a component without updating its local code and without compiling its dependencies
- `kdesrc-build --refresh-build component_name` : refresh the build directories before compiling
- `kdesrc-build --resume-from dependency_component component_name` : resume compilation from a specific dependency
- `kdesrc-build --run --exec executable_name component_name` : run a component with a specified executable name
- `kdesrc-build` : build all configured components
- `kdesrc-build --no-stop-on-failure component_name` : use system libraries in place of a component if it fails to build

## Categories
development, desktop, packages

## Links
- More information: https://manned.org/kdesrc-build
- tldr page: https://tldr.inbrowser.app/pages/linux/kdesrc-build
