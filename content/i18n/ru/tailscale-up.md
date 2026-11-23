# tailscale up

Подключать клиент к сети Tailscale. Все описанные здесь параметры можно изменить позже с помощью `tailscale set --option argument`. Используйте `--option=false` для отключения параметров, не требующих аргументов. Примечание: выполните `sudo tailscale set --operator $USER`, чтобы разрешить текущему пользователю выполнять эти команды.

## Usage
- `tailscale up` : подключиться к Tailscale
- `tailscale up --advertise-exit-node` : подключиться и предложить текущую машину в качестве выходного узла для интернет-трафика
- `tailscale up --exit-node ip_выходного_узла` : подключиться, используя конкретный узел для интернет-трафика
- `tailscale up --shields-up` : подключиться и заблокировать входящие подключения к текущему узлу
- `tailscale up --accept-dns=false` : подключиться и не принимать конфигурацию DNS из панели администрирования (по умолчанию `true`)
- `tailscale up --advertise-routes 10.0.0.0/24,10.0.1.0/24,...` : подключиться и настроить Tailscale в качестве маршрутизатора подсети
- `tailscale up --accept-routes` : подключиться и принять маршруты подсетей от Tailscale
- `tailscale up --reset` : сбросить неуказанные настройки к значениям по умолчанию и подключиться

## Links
- More information: https://tailscale.com/kb/1080/cli/#up
