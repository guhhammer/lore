# nuclei

Scanner de vulnerabilidades rápido e personalizável usando uma DSL simples baseada em YAML.

## Usage
- `nuclei -ut` : atualiza os templates do `nuclei` para a versão mais recente lançada (baixados em `~/nuclei-templates` no macOS/Linux ou `%USERPROFILE%\nuclei-templates` no Windows)
- `nuclei -tl -pt dns|file|http|headless|tcp|workflow|ssl|websocket|whois|code|javascript` : lista todos os templates por tipo específico de protocolo
- `nuclei -as -u example.com` : executa uma varredura web automática usando detecção de tecnologias do Wappalyzer para uma uRL/host alvo
- `nuclei -s high,critical,... -pt http -u https://example.com -me caminho/para/diretorio` : executa templates de protocolo do tipo HTTP de severidade específica, exportando os resultados em arquivos Markdown dentro de um diretório específico
- `nuclei -rl 150 -bs 25 -c 25 -silent -u https://example.com` : executa todos os templates com um limite de requisições personalizado, tamanho máximo de lote e saída silenciosa (apenas achados são exibidos)
- `nuclei -w workflows/wordpress-workflow.yaml -u https://example.com` : executa um fluxo de trabalho (workflow) específico integrado ao nuclei contra um alvo
- `nuclei -t caminho/para/nuclei-templates/http -u https://example.com -v -o caminho/para/resultados` : executa um ou mais templates específicos ou um diretório com templates com saída detalhada em `stderr`, gravando vulnerabilidades detectadas em um arquivo
- `nuclei -u https://example.com -ai "find admin login endpoints"` : usa um prompt de IA para gerar dinamicamente um template e escanear um alvo (requer a chave de API da nuvem da ProjectDiscovery configurada via `nuclei -auth`)

## Links
- More information: https://docs.projectdiscovery.io/opensource/nuclei/running
