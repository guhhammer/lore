# clj

Ferramenta de Clojure para iniciar um REPL ou invocar uma função com argumentos. Todas as opções podem ser definidas em um arquivo `deps.edn`.

## Usage
- `clj` : inicia um REPL
- `clj -X namespace/function_name` : executa uma função
- `clj -M -m namespace args` : executa a função principal (main) do namespace especificado
- `clj -P` : prepara um projeto resolvendo dependências, baixando bibliotecas, e criando / cacheando classpaths
- `clj -Sdeps '{:deps {nrepl {:mvn/version "0.7.0"} cider/cider-nrepl {:mvn/version "0.25.2"}\}\}' -m nrepl.cmdline --middleware '["cider.nrepl/cider-middleware"]' --interactive` : inicia um servidor nREPL com o middleware CIDER
- `clj -Sdeps '{:deps {org.clojure/clojurescript {:mvn/version "1.10.758"}\}\}' -m cljs.main -r` : inicia um REPL para ClojureScript e abre um navegador web

## Links
- More information: https://clojure.org/guides/deps_and_cli
