# chromium

Navegador web de código aberto desenvolvido e mantido principalmente pela Google.

## Usage
- `chromium https://example.com|caminho/para/arquivo.html` : abre uma URL ou arquivo específico
- `chromium --incognito example.com` : abre no modo de navegação anônima (incógnito)
- `chromium --new-window example.com` : abre em uma nova janela
- `chromium --app=https://example.com` : abre no modo aplicativo (sem barra de tarefas, barra de URL, botões, etc.)
- `chromium --proxy-server="socks5://hostname:66" example.com` : usa um servidor proxy
- `chromium --user-data-dir=caminho/para/diretório` : abre com um diretório de perfil customizado
- `chromium --user-data-dir=caminho/para/diretório --disable-web-security` : abre sem validação CORS (útil para testar uma API)
- `chromium --auto-open-devtools-for-tabs` : abre com uma janela DevTools para cada aba aberta

## Links
- More information: https://www.chromium.org/developers/how-tos/run-chromium-with-flags/
