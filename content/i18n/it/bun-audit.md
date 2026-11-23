# bun audit

Controlla i pacchetti installati per vulnerabilità di sicurezza conosciute.

## Usage
- `bun audit` : controlla tutte le dipendenze di un progetto con un file `bun.lock`
- `bun audit --audit-level low|moderate|high|critical` : mostra solo le vulnerabilità con gravità pari o superiore al livello specificato
- `bun audit --prod` : controlla solo le dipendenze di produzione
- `bun audit --ignore CVE-XXXX-YYYY` : ignora un CVE specifico
- `bun audit --json` : stampa il report JSON grezzo

## Links
- More information: https://bun.com/docs/pm/cli/audit
