# pacman-mirrors

Gera uma lista de mirrors do pacman para o Manjaro Linux. Toda execução do pacman-mirrors requer que você sincronize seu bando de dados e atualize seu sistema usado `sudo pacman -Syyu`.

## Usage
- `sudo pacman-mirrors --fasttrack` : gera uma lista de mirrors usando as configurações padrão
- `pacman-mirrors --status` : obtém o status dos mirrors atuais
- `pacman-mirrors --get-branch` : exibe a branch atual
- `sudo pacman-mirrors --api --set-branch stable|unstable|testing` : muda para uma branch diferente
- `sudo pacman-mirrors --geoip` : gera uma lista de mirror, usando apenas mirrors em seu país

## Links
- More information: https://wiki.manjaro.org/index.php?title=Pacman-mirrors
