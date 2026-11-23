# bun install

Installeer JavaScript-afhankelijkheden voor een project vanuit `package.json`.

## Usage
- `bun i` : installeer alle afhankelijkheden vermeld in `package.json`
- `bun i pakket_naam@versie` : installeer een enkel pakket (dit is een alias voor `bun add`)
- `bun i -g pakket_naam` : installeer een pakket globaal
- `bun i -p` : installeer alleen productieafhankelijkheden (slaat `devDependencies` over)
- `bun i --frozen-lockfile` : installeer afhankelijkheden precies vanuit het `bun.lockb` lockbestand (bevroren lockbestand)
- `bun i -f` : forceer het opnieuw downloaden van alle pakketten uit het register, waarbij de cache wordt genegeerd

## Links
- More information: https://bun.com/docs/pm/cli/install
