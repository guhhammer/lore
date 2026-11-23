# bun install

Installa le dipendenze JavaScript di un progetto da `package.json`.

## Usage
- `bun i` : installa tutte le dipendenze elencate in `package.json`
- `bun i nome_pacchetto@versione` : installa un singolo pacchetto (alias di `bun add`)
- `bun i -g nome_pacchetto` : installa un pacchetto globalmente
- `bun i -p` : installa solo le dipendenze di produzione (salta `devDependencies`)
- `bun i --frozen-lockfile` : installa le dipendenze esattamente dal lockfile `bun.lockb` (frozen lockfile)
- `bun i -f` : forza il download di tutti i pacchetti dal registro, ignorando la cache

## Links
- More information: https://bun.com/docs/pm/cli/install
