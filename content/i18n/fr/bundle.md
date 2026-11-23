# bundle

Gestionnaire de dépendances pour le langage de programmation Ruby.

## Usage
- `bundle install` : installe toutes les gems définies dans le `Gemfile` attendu dans le répertoire de travail
- `bundle exec commande arguments` : exécute une commande dans le contexte du bundle actuel
- `bundle update --all` : met à jour toutes les gems selon les règles définies dans le `Gemfile` et régénère le fichier `Gemfile.lock`
- `bundle update nom_gem1 nom_gem2 ...` : met à jour une ou plusieurs gem(s) spécifique(s) définie(s) dans le `Gemfile`
- `bundle update --patch nom_gem1 nom_gem2 ...` : met à jour une ou plusieurs gem(s) spécifique(s) définie(s) dans le `Gemfile` mais seulement vers la prochaine version de patch
- `bundle update --group nom_groupe` : met à jour toutes les gem(s) du groupe donné dans le `Gemfile`
- `bundle outdated` : liste les gem(s) installée(s) dans le `Gemfile` avec les nouvelles versions disponibles
- `bundle gem nom_gem` : crée un nouveau squelette de gem

## Links
- More information: https://bundler.io/man/bundle.1.html
