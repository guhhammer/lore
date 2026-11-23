# clj

Herramienta de Clojure para usar una interfaz interactiva (REPL) o invocar una función con datos. Todas las opciones se pueden definir en un archivo `deps.edn`.

## Usage
- `clj` : inicia una REPL (interfaz interactiva)
- `clj -X namespace/nombre_de_función` : ejecuta una función
- `clj -M -m namespace argumentos` : ejecuta la función principal de un espacio de nombres (namespace) especificado
- `clj -P` : prepara un proyecto resolviendo dependencias, descargando bibliotecas y haciendo/cacheando rutas de clases (classpaths)
- `clj -Sdeps '{:deps {nrepl {:mvn/version "0.7.0"} cider/cider-nrepl {:mvn/version "0.25.2"}\}\}' -m nrepl.cmdline --middleware '["cider.nrepl/cider-middleware"]' --interactive` : inicia un servidor nREPL con el software intermedio (middleware) CIDER
- `clj -Sdeps '{:deps {org.clojure/clojurescript {:mvn/version "1.10.758"}\}\}' -m cljs.main -r` : inicia un REPL de ClojureScript y abre un navegador web

## Links
- More information: https://clojure.org/guides/deps_and_cli
