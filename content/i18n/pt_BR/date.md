# date

Define ou exibe a data do sistema.

## Usage
- `date +%c` : exibe a data atual usando o formato padrão de localidade
- `date -u +%Y-%m-%dT%H:%M:%SZ` : exibe a data atual em UTC, usando o formato ISO 8601
- `date +%s` : exibe a data atual em Unix timestamp - segundos desde 00:00:00 UTC de 1 de janeiro de 1970 (Unix epoch)
- `date -d @1473305798` : converte uma data especificada como Unix timestamp para o formato padrão
- `date -d "2018-09-01 00:00" +%s -u` : converte uma determinada data pra Unix timestamp
- `date --rfc-3339 s` : exibe a data atual usando o formato RFC-3339 (`YYYY-MM-DD hh:mm:ss+TZ`)
- `date 093023592021.59` : define a data atual usando o formato `MMDDhhmmYYYY.ss` (`YYYY` e `.ss` são opcionais)
- `date +%V` : exibe o número da semana ISO atual

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/date-invocation.html
