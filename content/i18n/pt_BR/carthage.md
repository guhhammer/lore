# carthage

Ferramenta de gerenciamento de dependências para aplicativos Cocoa.

## Usage
- `carthage update` : baixa a versão mais recente de todas as dependências mencionadas no Cartfile e realiza o build delas
- `carthage update --platform ios` : atualiza as dependências, e faz build apenas para o iOS
- `carthage update --no-build` : atualiza as dependências, sem realizar build de nenhuma delas
- `carthage bootstrap` : faz o download e rebuild da versão atual das dependências (sem atualizá-las)
- `carthage build dependência` : faz o rebuild de uma dependência específica

## Links
- More information: https://github.com/Carthage/Carthage
