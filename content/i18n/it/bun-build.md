# bun build

Bundla file JavaScript e TypeScript con il bundler nativo veloce di Bun.

## Usage
- `bun build percorso/dell/entry.ts --outfile percorso/del/output.js` : bundla un entry point in un singolo file di output
- `bun build percorso/dell/entry1.ts percorso/dell/entry2.ts ... --outdir percorso/del/dist` : bundla più entry point in una directory di output
- `bun build percorso/dell/entry.ts --outfile percorso/del/output.js --sourcemap` : bundla con sourcemap per il debug
- `bun build percorso/dell/entry.ts --outfile percorso/del/output.js --minify` : bundla con minificazione per la produzione
- `bun build percorso/dell/entry.ts --outfile percorso/del/output.js --target browser|bun|node` : bundla per un ambiente target specifico
- `bun build percorso/dell/entry.ts --compile --outfile percorso/dell/executable` : bundla in un eseguibile standalone
- `bun build percorso/dell/entry.ts --outfile percorso/del/output.js --watch` : osserva i cambiamenti dei file e ricostruisci automaticamente
- `bun build percorso/dell/entry.ts --outfile percorso/del/output.js -e react react-dom` : bundla con dipendenze esterne non incluse nell'output

## Links
- More information: https://bun.com/docs/bundler
