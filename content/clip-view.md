# clip-view

Command Line Interface Pages (CLIP) render. Render for a TlDr-like project with much a more extensive syntax and several render modes.

## Usage
- `clip-view path/to/page1.clip path/to/page2.clip ...` : render specific local pages
- `clip-view page_name1 page_name2 ...` : render specific remote pages
- `clip-view --render tldr|tldr-colorful|docopt|docopt-colorful page_name1 page_name2 ...` : render pages by a specific render
- `clip-view --theme path/to/local_theme.yaml|remote_theme_name page_name1 page_name2 ...` : render pages with a specific color theme
- `clip-view --clear-page|theme-cache` : clear a page or theme cache
- `clip-view --help` : display help
- `clip-view --version` : display version

## Categories
network, text, development

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/clip-view
