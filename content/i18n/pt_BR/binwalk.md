# binwalk

Ferramenta de análise de Firmware.

## Usage
- `binwalk caminho/para/binário` : escaneia um arquivo binário
- `binwalk -e -C diretório_do_destino caminho/para/binário` : extrai arquivos de um binário, especificando a saída do diretório
- `binwalk -e -M -d 2 caminho/para/binário` : extrai recursivamente arquivos de um binário limitando a profundidade da recursão para 2
- `binwalk -D 'png image:png' caminho/para/binário` : extrai arquivos de um binário com uma assinatura específica
- `binwalk -E -J caminho/para/binário` : analisa a entropia de um binário, salvando o gráfico com o mesmo nome que o binário e a extensão `.png`
- `binwalk -E -B -A caminho/para/binário` : combina entropia, assinatura e análise dos código de operações em um comando só

## Links
- More information: https://manned.org/binwalk
