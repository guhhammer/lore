# kde-builder

Easily build KDE components from its source repositories. Drop-in replacement for `kdesrc-build`.

## Usage
- `kde-builder --initial-setup` : initialize `kde-builder`
- `kde-builder component_name1 component_name2 ...` : compile a KDE component and its dependencies from the source (use `workspace` to compile Plasma desktop)
- `kde-builder -SD component_name` : compile a component without updating its local code and without compiling its dependencies
- `kde-builder -r component_name` : refresh the build directories before compiling
- `kde-builder -f dependency_component component_name` : resume compilation from a specific dependency
- `kde-builder --run executable_name` : run a component with a specified executable name
- `kde-builder --install-login-session-only` : install login session
- `kde-builder --no-stop-on-failure component_name` : use system libraries in place of a component if it fails to build

## Categories
desktop, development, packages

## Links
- More information: https://kde-builder.kde.org/en/cmdline/supported-cmdline-params.html
- tldr page: https://tldr.inbrowser.app/pages/linux/kde-builder
