# tailscale funnel

Открывать доступ к локальному серверу из интернета с помощью Tailscale.

## Usage
- `tailscale funnel /путь/к/файлу_или_каталогу` : открыть доступ к локальному файлу или каталогу на переднем плане
- `tailscale funnel 3000` : открыть доступ к HTTP-серверу, работающему на 127.0.0.1:3000, на переднем плане
- `tailscale funnel --bg 3000` : открыть доступ к HTTP-серверу, работающему на 127.0.0.1:3000, в фоновом режиме
- `tailscale funnel https+insecure://localhost:8443` : открыть доступ к HTTPS-серверу с недействительными или самоподписанными сертификатами на <https://localhost:8443>
- `tailscale funnel status` : показать текущую конфигурацию
- `tailscale funnel status --json` : показать текущую конфигурацию в формате JSON
- `tailscale funnel reset` : сбросить текущую конфигурацию

## Links
- More information: https://tailscale.com/kb/1311/tailscale-funnel
