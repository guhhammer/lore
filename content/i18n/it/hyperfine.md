# hyperfine

Strumento di benchmarking con interfaccia CLI.

## Usage
- `hyperfine 'make'` : esegui un benchmark di base, eseguendo almeno 10 esecuzioni
- `hyperfine 'make target1' 'make target2'` : esegui un benchmark comparativo
- `hyperfine -m 7 'make'` : modifica il numero minimo di esecuzioni di benchmark
- `hyperfine -w 5 'make'` : esegui benchmark con periodo di riscaldamento
- `hyperfine -p 'make clean' 'make'` : esegui un comando prima di ogni esecuzione di benchmark (per cancellare le cache, etc.)
- `hyperfine -p 'make clean' -P num_threads 1 10 'make --jobs {num_threads}'` : esegui un benchmark in cui un singolo parametro cambia per ogni esecuzione

## Links
- More information: https://manned.org/hyperfine
