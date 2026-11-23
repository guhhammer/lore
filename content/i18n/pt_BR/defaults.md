# defaults

Lê e grava a configuração do usuário do macOS para aplicativos.

## Usage
- `defaults read "aplicativo" "opção"` : lê os padrões do sistema para uma opção do aplicativo
- `defaults read -app "aplicativo" "opção"` : lê os valores padrão para uma opção do aplicativo
- `defaults find "palavra_chave"` : pesquisa uma palavra-chave em nomes de domínio, chaves, e valores
- `defaults write "aplicativo" "opção" -tipo valor` : grava o valor padrão de uma opção do aplicativo
- `defaults write com.apple.Dock expose-animation-duration -float 0.1` : acelera as animações do Mission Control
- `defaults delete "aplicativo"` : exclui todos os padrões de um aplicativo

## Links
- More information: https://keith.github.io/xcode-man-pages/defaults.1.html
