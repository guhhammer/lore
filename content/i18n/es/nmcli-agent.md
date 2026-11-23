# nmcli agent

Ejecuta `nmcli` como agente secreto de NetworkManager o agente polkit. Este subcomando también se puede llamar con `nmcli a`.

## Usage
- `nmcli a s` : registra `nmcli` como agente secreto y escucha solicitudes secretas
- `nmcli a p` : registra `nmcli` como agente polkit y escucha solicitudes de autorización
- `nmcli a a` : registra `nmcli` como agente secreto y agente de polkit
