# gdu

Analisador de uso de disco com interface de console.

## Usage
- `gdu` : exibe interativamente o uso de disco do diretório atual
- `gdu caminho/para/diretório` : exibe interativamente o uso de disco de um determinado diretório
- `gdu --show-disks` : exibe interativamente o uso de disco de todos os discos montados
- `gdu --ignore-dirs caminho/para/diretório1,caminho/para/diretório2,...` : exibe interativamente o uso de disco do diretório atual, mas ignora alguns subdiretórios
- `gdu --ignore-dirs-pattern '.*[abc]+'` : ignora caminhos por expressão regular
- `gdu --no-hidden` : ignora diretórios ocultos
- `gdu --non-interactive caminho/para/diretório` : imprime apenas o resultado, não entra no modo interativo
- `gdu --no-progress caminho/para/diretório` : não mostra o progresso no modo não interativo (útil em scripts)

## Links
- More information: https://github.com/dundee/gdu#usage
