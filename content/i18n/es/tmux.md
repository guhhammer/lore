# tmux

Multiplexor de terminal. Permite múltiples sesiones con ventanas, paneles y más.

## Usage
- `tmux` : inicia una nueva sesión
- `tmux new -s nombre` : inicia una nueva sesión con nombre
- `tmux ls` : lista las sesiones existentes
- `tmux a` : adjunta a la última sesión utilizada
- `<Ctrl b><d>` : separa la sesión actual (dentro de una sesión tmux)
- `<Ctrl b><c>` : crea una nueva ventana (dentro de una sesión tmux)
- `<Ctrl b><w>` : cambia entre sesiones y ventanas (dentro de una sesión tmux)
- `tmux kill-session -t nombre` : da de baja una sesión por su nombre

## Links
- More information: https://github.com/tmux/tmux
