# date

Imposta o mostra data e ora di sistema.

## Usage
- `date +"%c"` : mostra la data corrente utilizzando il formato predefinito della locale corrente
- `date -u +"%Y-%m-%dT%H:%M:%SZ"` : mostra la data corrente in UTC e formato ISO 8601
- `date +%s` : mostra la data corrente come timestamp Unix (secondi dall'epoca Unix)
- `date -d @1473305798` : mostra una specifica data (rappresentata come timestamp Unix) utilizzando il formato predefinito
- `date -d "2018-09-01 00:00" +%s --utc` : converti una specifica data in un timestamp Unix

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/date-invocation.html
