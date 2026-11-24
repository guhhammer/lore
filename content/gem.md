# gem

A package manager for the Ruby programming language.

## Usage
- `gem search regex -a` : search for remote gem(s) and show all available versions
- `gem install gem_name` : install the latest version of a gem
- `gem install gem_name -v 1.0.0` : install a specific version of a gem
- `gem install gem_name -v '~> 1.0'` : install the latest matching (SemVer) version of a gem
- `gem update gem_name` : update a gem
- `gem list` : list all local gems
- `gem uninstall gem_name` : uninstall a gem
- `gem uninstall gem_name -v 1.0.0` : uninstall a specific version of a gem

## Categories
packages, development

## Links
- More information: https://guides.rubygems.org/command-reference/
- tldr page: https://tldr.inbrowser.app/pages/common/gem
- Wikipedia: https://en.wikipedia.org/wiki/RubyGems
