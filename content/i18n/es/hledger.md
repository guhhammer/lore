# hledger

Una aplicación de contabilidad en texto plano que es robusta y fácil de usar.

## Usage
- `hledger add` : registra nuevas transacciones interactivamente, guardándolas en el archivo de diario por defecto
- `hledger import ruta/a/banco.csv` : importa nuevas transacciones de `banco.csv`, usando `banco.csv.rules` para convertir
- `hledger print -f ruta/a/precios-2024.journal -f ruta/a/precios-2023.journal` : imprime todas las transacciones, leyendo múltiples archivos de diario específicos
- `hledger accounts -t --types` : muestra todas las cuentas, como jerarquía, y sus tipos
- `hledger bs -E -t --no-elide` : muestra saldos de cuenta de activos y pasivos, incluyendo ceros, jerárquicamente
- `hledger is -M -T -A --sort -2` : muestra ingresos/gastos/totales mensuales, los más grandes primero, resumido a 2 niveles
- `hledger areg assets:bank:checking` : muestra las transacciones de la cuenta `assets:bank:checking` y su saldo actual
- `hledger print assets:cash | hledger -f - -I aregister expenses:food` : muestra el monto gastado en comida desde la cuenta `assets:cash`

## Links
- More information: https://hledger.org/hledger.html
