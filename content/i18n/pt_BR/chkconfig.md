# chkconfig

Gerencia o runlevel dos serviços no CentOS 6.

## Usage
- `chkconfig --list` : exibe os serviços com os respectivos runlevels
- `chkconfig --list ntpd` : exibe o runlevel de um serviço
- `chkconfig sshd on` : habilita o início de um serviço durante o processo de boot
- `chkconfig --level 2345 sshd on` : habilita o início do serviço durante o processo de boot para os runlevels 2, 3, 4 e 5
- `chkconfig ntpd off` : desabilita a inicialização de um determinado serviço durante o processo de boot
- `chkconfig --level 3 ntpd off` : desabilita a inicialização de um determinado serviço durante o processo de boot para o runlevel 3

## Links
- More information: https://manned.org/chkconfig
