# pgbench

Ejecuta una prueba de referencia (benchmark test) en PostgreSQL.

## Usage
- `pgbench --initialize --scale=50 nombre_base_de_datos` : inicia una base de datos con un factor de escalamiento de 50 veces el tamaño predeterminado
- `pgbench --client=10 --jobs=2 --transactions=10000 nombre_base_de_datos` : hace una prueba de referencia a una base de datos con 10 clientes, 2 hilos de trabajo y 10.000 transacciones por cliente

## Links
- More information: https://www.postgresql.org/docs/current/pgbench.html
