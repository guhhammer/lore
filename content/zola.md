# zola

A static site generator in a single binary with everything built-in.

## Usage
- `zola init my_site` : create the directory structure used by Zola at the given directory
- `zola build` : build the whole site in the `public` directory after deleting it
- `zola build --output-dir path/to/output_directory/` : build the whole site into a different directory
- `zola serve` : build and serve the site using a local server (default is `127.0.0.1:1111`)
- `zola check` : build all pages just like the build command would, but without writing any of the results to disk

## Categories
development, files

## Links
- More information: https://www.getzola.org/documentation/getting-started/cli-usage/
- tldr page: https://tldr.inbrowser.app/pages/common/zola
