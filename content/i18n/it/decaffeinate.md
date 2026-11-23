# decaffeinate

Converti script CoffeScript in JavaScript moderno.

## Usage
- `decaffeinate percorso/del/file.coffee` : converti un file CoffeeScript in JavaScript
- `decaffeinate --use-cs2 percorso/del/file.coffee` : converti un file CoffeeScript v2 in JavaScript
- `decaffeinate --use-js-modules percorso/del/file.coffee` : converti `require` e `module.exports` in `import` ed `export`
- `decaffeinate --loose-js-modules percorso/del/file.coffee` : converti un file CoffeeScript, permettendo di esportare nomi

## Links
- More information: https://www.npmjs.com/package/decaffeinate#common-options
