# pygbag

Package Pygame projects as WebAssembly to run in web browsers.

## Usage
- `pygbag path/to/project_folder` : package a Pygame project and start a local test server
- `python -m pygbag path/to/project_folder` : package using Python module syntax
- `pygbag path/to/project_folder --build` : package and build without starting the test server
- `pygbag path/to/project_folder --template template_name.tmpl` : package with a specific template
- `pygbag path/to/project_folder --archive` : package and create a ZIP archive for <https://itch.io>
- `pygbag path/to/project_folder --no_opt` : package with optimization turned off
- `pygbag path/to/project_folder --port 8080` : specify a custom port for the test server
- `pygbag -h` : display help

## Categories
packages, development, files

## Links
- More information: https://github.com/pygame-web/pygbag#pygbag
- tldr page: https://tldr.inbrowser.app/pages/common/pygbag
