# bundle

Administrador de dependencias para el lenguaje de programación Ruby.

## Usage
- `bundle install` : instala todas las gemas (gems) definidas en el archivo `Gemfile` en el directorio de trabajo
- `bundle exec comando argumentos` : ejecuta un comando en el contexto del paquete (bundle) actual
- `bundle update --all` : actualiza todas las gemas (gems) definidas por las reglas en el `Gemfile` y regenera `Gemfile.lock`
- `bundle update gema1 gema2 ...` : actualiza una o más gemas (gems) específicas definidas en el `Gemfile`
- `bundle update --patch gema1 gema2 ...` : actualiza una o más gemas (gems) específicas definidas en el `Gemfile` pero solo a la siguiente versión parche (patch)
- `bundle update --group development` : actualiza todas las gemas (gems) dentro de un grupo dado en el `Gemfile`
- `bundle outdated` : lista las gemas (gems) instaladas con nuevas versiones disponibles definidas en el `Gemfile`
- `bundle gem gema` : crea una nueva estructura de gema (gem)

## Links
- More information: https://bundler.io/man/bundle.1.html
