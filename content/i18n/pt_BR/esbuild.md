# esbuild

Empacotador e minificador JavaScript construído para velocidade.

## Usage
- `esbuild --bundle caminho/para/arquivo.js` : empacota uma aplicação JavaScript e imprime para `stdout`
- `esbuild <caminho/para/arquivo.jsx --bundle --outfile=caminho/para/saída.js` : empacota uma aplicação JSX de `stdin`
- `esbuild --bundle --define:process.env.NODE_ENV=\"production\" --minify --sourcemap caminho/para/arquivo.js` : empacota e reduz uma aplicação JSX com mapas de origem no modo `production`
- `esbuild --bundle --minify --sourcemap --target=chrome58,firefox57,safari11,edge16 caminho/para/arquivo.jsx` : empacota uma aplicação JSX para uma lista de navegadores separados por vírgulas
- `esbuild --bundle --platform=node --target=node12 caminho/para/arquivo.js` : empacota uma aplicação JavaScript para uma versão específica do node
- `esbuild --bundle app.js --loader:.js=jsx caminho/para/arquivo.js` : empacota uma aplicação JavaScript habilitando a sintaxe JSX em arquivos `.js`
- `esbuild --bundle --serve=porta --outfile=index.js caminho/para/arquivo.js` : empacota e serve uma aplicação JavaScript em um servidor HTTP
- `esbuild --bundle --outdir=caminho/para/diretório_de_saída caminho/para/arquivo1 caminho/para/arquivo2 ...` : empacota uma lista de arquivos em um diretório de saída

## Links
- More information: https://esbuild.github.io/api/#general-options
