# systemsetup

Configura as definições de Preferencias do Sistema da máquina.

## Usage
- `systemsetup -setremotelogin on` : ativa autenticação remota (SSH)
- `systemsetup -settimezone "Europe/Lisbon" -setnetworktimeserver 2.pt.pool.ntp.org -setusingnetworktime on` : ativa o serviço de hora de rede com um fuso horário e servidor específico
- `systemsetup -setsleep off -setrestartpowerfailure on -setrestartfreeze on` : coloca a máquina sem dormir, reiniciando automaticamente em falta de energia ou pânico do núcleo do sistema
- `systemsetup -liststartupdisks` : lista os discos de inicialização validos
- `systemsetup -setstartupdisk caminho` : especifica um novo disco de inicialização
