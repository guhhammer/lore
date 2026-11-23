# bundle

Dependency manager voor de Ruby-programmeertaal.

## Usage
- `bundle install` : installeer alle gems gedefinieerd in de `Gemfile`, welke verwacht wordt in de huidige map
- `bundle exec commando argumenten` : voer een commando uit in de context van de huidige bundle
- `bundle update --all` : update alle gems volgens de regels gedefinieerd in de `Gemfile` en regenereer de `Gemfile.lock`
- `bundle update gem_naam1 gem_naam2 ...` : update een of meerdere specifieke gem(s) gedefinieerd in de `Gemfile`
- `bundle update --patch gem_naam1 gem_naam2 ...` : update een of meerdere specifieke gem(s) gedefinieerd in de `Gemfile` maar alleen naar de volgende patch-versie
- `bundle update --group development` : update alle gems binnen de gegeven groep in de `Gemfile`
- `bundle outdated` : toon de geïnstalleerde gems in de `Gemfile` welke nieuwere versies beschikbaar hebben
- `bundle gem gem_naam` : maak een nieuw gem-skelet

## Links
- More information: https://bundler.io/man/bundle.1.html
