# systemctl cancel

Cancela uno o más trabajos pendientes en el administrador del sistema o de usuario.

## Usage
- `systemctl cancel id_trabajo` : cancela un trabajo por su ID numérico
- `systemctl cancel id_trabajo1 id_trabajo2 ...` : cancela múltiples trabajos
- `systemctl cancel` : cancela todos los trabajos pendientes
- `systemctl cancel id_trabajo --user` : cancela un trabajo en el administrador de servicios de usuario
