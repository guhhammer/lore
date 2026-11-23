# bundle

Gestore delle dipendenze per il linguaggio Ruby.

## Usage
- `bundle install` : installa tutte le gem definite nel `Gemfile` nella directory corrente
- `bundle exec comando argomenti` : esegue un comando nel contesto del bundle corrente
- `bundle update --all` : aggiorna tutte le gem secondo le regole del `Gemfile` e rigenera `Gemfile.lock`
- `bundle update nome_gem1 nome_gem2 ...` : aggiorna una o più gem specifiche definite nel `Gemfile`
- `bundle update --patch nome_gem1 nome_gem2 ...` : aggiorna una o più gem specifiche solo alla prossima versione patch
- `bundle update --group development` : aggiorna tutte le gem del gruppo specificato nel `Gemfile`
- `bundle outdated` : elenca le gem installate del `Gemfile` con versioni più recenti disponibili
- `bundle gem nome_gem` : crea uno scheletro per una nuova gem

## Links
- More information: https://bundler.io/man/bundle.1.html
