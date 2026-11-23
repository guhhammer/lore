# hledger balance

Un informe "sumatorio" flexible y de propósito general que muestra cuentas con algún tipo de dato numérico. Puede tratarse de cambios de saldo por periodo, saldos finales, rendimiento presupuestario, plusvalías latentes, etc.

## Usage
- `hledger bal` : muestra el cambio de saldo en todas las cuentas de todas las contabilizaciones a lo largo de todo el tiempo
- `hledger bal gastos -t -2` : muestra el cambio de saldo en las cuentas denominadas `*gastos*`, como un árbol, resumiendo solo los dos niveles superiores
- `hledger bal gastos -M -T -A -S --budget` : muestra los gastos de cada mes, y sus totales y medias, ordenados por total; y sus objetivos presupuestarios mensuales
- `hledger bal type:X -MTAS --budget -t -2 --no-elide` : similar a la anterior, de forma más corta, comparando las cuentas por tipo de `Gastos`, como un árbol de dos niveles sin aplastar las cuentas aburridas
- `hledger bal -H -p 'trimestral en 2024' activos pasivos` : muestra saldos finales (incluidos los de contabilizaciones anteriores a la fecha de inicio), trimestrales en 2024, en cuentas denominadas `*activos*` o `*pasivos*`
- `hledger bal -HQ date:2024 type:AL -ES -3` : similar a la anterior, de un modo más breve; también muestra saldos en cero, ordena por total y resume a tres niveles
- `hledger bal -HVQ activos:inversiones` : muestra el valor de mercado de los activos de inversión en moneda base al final de cada trimestre
- `hledger bal --gain -Q activos:inversiones not:criptomoneda` : muestra las ganancias/pérdidas de capital no realizadas por cambios en el precio de mercado en cada trimestre, para activos de inversión que no sean criptomonedas

## Links
- More information: https://hledger.org/hledger.html#balance
