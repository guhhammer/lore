# xcodes runtimes

Gerencia runtimes do Simulador Xcode.

## Usage
- `xcodes runtimes --include-betas` : lista todos os runtimes do Simulador disponíveis
- `xcodes runtimes download nome_do_runtime` : baixa um runtime do Simulador
- `xcodes runtimes install nome_do_runtime` : baixa e instala um runtime do Simulador
- `xcodes runtimes download|install "iOS|watchOS|tvOS|visionOS versão_runtime"` : baixa/instala um runtime do Simulador para a versão iOS/watchOS/tvOS/visionOS especificada (diferencia maiúsculo de minúsculo)
- `xcodes runtimes download|install nome_runtime --directory caminho/para/diretório` : define um local específico para onde o pacote do runtime será baixado primeiro (o padrão é `~/Downloads`)
- `xcodes runtimes install nome_runtime --keep-archive` : não exclui o pacote baixado quando o Simulador é instalado com sucesso

## Links
- More information: https://github.com/xcodesorg/xcodes#commands
