# env

Vis miljøet eller kjør et program i et modifisert miljø.

## Usage
- `env` : vis miljøet
- `env program` : kjør et program. Ofte brukt i skript etter shebang (#!) for å slå opp stien til programmet
- `env -i program` : slett miljøet og kjør et program
- `env -u variabel program` : fjern variabel fra miljøet og kjør et program
- `env variabel=verdi program` : angi en variabel og kjør et program
- `env variabel1=verdi variabel2=verdi variabel3=verdi program` : angi flere variabler og kjør et program

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/env-invocation.html
