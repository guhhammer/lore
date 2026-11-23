# bundle

Gerenciador de dependências da linguagem de programação Ruby.

## Usage
- `bundle install` : instala todas as gemas definidas no `Gemfile` esperadas no diretório de trabalho
- `bundle exec comando argumentos` : executa um comando no contexto do pacote atual
- `bundle update --all` : atualiza todas as gemas respeitando as regras definidas no `Gemfile` e recria o arquivo `Gemfile.lock`
- `bundle update nome_da_gema1 nome_da_gema2 ...` : atualiza uma ou mais gema(s) específicas definidas no `Gemfile`
- `bundle update --patch nome_da_gema1 nome_da_gema2 ...` : atualiza uma ou mais gema(s) específicas definidas no `Gemfile` mas somente para a próxima versão de patch
- `bundle update --group desenvolvimento` : atualiza todas as gemas do grupo especificado no `Gemfile`
- `bundle outdated` : lista gemas instaladas no `Gemfile` com novas versões disponíveis
- `bundle gem nome_da_gema` : cria o esqueleto do projeto de uma nova gema

## Links
- More information: https://bundler.io/man/bundle.1.html
