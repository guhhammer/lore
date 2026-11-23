# hledger-ui

Una interfaz de terminal (TUI) para `hledger`, una aplicación de contabilidad de texto plano robusta y fácil de usar.

## Usage
- `hledger-ui` : se inicia en la pantalla del menú principal, leyendo desde el archivo de diario predeterminado
- `hledger-ui --theme terminal|greenterm|dark` : se inicia con un tema de colores diferente
- `hledger-ui --bs -t -3` : se inicia con la pantalla de cuentas del balance, mostrando la jerarquía hasta el nivel 3
- `hledger-ui --register assets:bank:checking -C -w` : se inicia en la pantalla de la cuenta, mostrando las transacciones liquidadas, y se recarga al producirse un cambio
- `hledger-ui -f ruta/a/2024.journal -f ruta/a/2024-prices.journal --value now` : lee dos archivos de diario y muestra los importes como valor actual cuando se conozca
- `hledger-ui --info` : muestra el manual en formato Info, si es posible
- `hledger-ui -h` : muestra la ayuda

## Links
- More information: https://hledger.org/hledger-ui.html
