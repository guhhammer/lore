# bundle

Dependency manager for the Ruby programming language.

## Usage
- `bundle install` : install all gems defined in the `Gemfile` expected in the working directory
- `bundle exec command arguments` : execute a command in the context of the current bundle
- `bundle update --all` : update all gems by the rules defined in the `Gemfile` and regenerate `Gemfile.lock`
- `bundle update gem_name1 gem_name2 ...` : update one or more specific gem(s) defined in the `Gemfile`
- `bundle update --patch gem_name1 gem_name2 ...` : update one or more specific gems(s) defined in the `Gemfile` but only to the next patch version
- `bundle update --group development` : update all gems within the given group in the `Gemfile`
- `bundle outdated` : list installed gems in the `Gemfile` with newer versions available
- `bundle gem gem_name` : create a new gem skeleton

## Categories
development

## Links
- More information: https://bundler.io/man/bundle.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/bundle
