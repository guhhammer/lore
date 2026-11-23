# caddy

Um servidor web open source, pronto para empresas, com HTTPS automático, escrito em Go.

## Usage
- `caddy run` : inicia Caddy em primeiro plano
- `caddy run --config caminho/para/arquivoCaddy` : inicia Caddy com um arquivo Caddy específico
- `caddy start` : inicia Caddy no plano de fundo
- `caddy stop` : para um processo Caddy em plano de fundo
- `caddy file-server --listen :8000 --browse` : executa um servidor de arquivo simples na porta especificada, com uma interface navegável
- `caddy reverse-proxy --from :80 --to localhost:8000` : executa um servidor proxy reverso

## Links
- More information: https://caddyserver.com/docs/command-line
