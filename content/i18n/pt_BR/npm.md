# npm

Gerenciador de pacotes JavaScript e Node.js. Gerencia projetos Node.js e suas dependências de módulos.

## Usage
- `npm init -y` : cria um arquio `package.json` com os valores padrões (omita `--yes` para torná-lo interativo)
- `npm i` : baixa todos os pacotes listados como dependências em `package.json`
- `npm i pacote@versão` : baixa uma versão específica de um pacote e o adiciona à lista de dependências em `package.json`
- `npm i pacote -D` : baixa a última versão de um pacote e o adiciona à lista de dependências de desenvolvimento em `package.json`
- `npm i pacote -g` : baixa a última versão de um pacote e o instala globalmente
- `npm r pacote` : desinstala um pacote e o remove da lista de dependências em `package.json`
- `npm ls` : lista todas as dependências instaladas localmente
- `npm ls -g --depth 0` : lista todos os pacotes de nível superior instalados globalmente

## Links
- More information: https://docs.npmjs.com/cli/npm/
