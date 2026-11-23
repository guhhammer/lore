# hugo

Template-based static site generator. Uses modules, components, and themes. Some subcommands such as `server` have their own usage documentation.

## Usage
- `hugo new site path/to/site` : create a new Hugo site
- `hugo new theme theme_name` : create a new Hugo theme (themes may also be downloaded from <https://themes.gohugo.io/>)
- `hugo new section_name/page_name` : create a new page
- `hugo` : build a site to the `public` directory
- `hugo -D` : build a site including pages that are marked as a "draft"
- `hugo server --bind local_ip -b http://local_ip` : build a site on your local IP
- `hugo -d path/to/destination` : build a site to a given directory
- `hugo server` : build a site, start up a webserver to serve it, and automatically reload when pages are edited

## Categories
development, files, network

## Links
- More information: https://gohugo.io/commands/
- tldr page: https://tldr.inbrowser.app/pages/common/hugo
