# tmux

Multiplexeur de terminaux. Permet plusieurs sessions avec fenêtres, panneaux, et plus encore.

## Usage
- `tmux` : démarre une nouvelle session
- `tmux new -s nom` : démarre une nouvelle session nommée
- `tmux ls` : liste les sessions existantes
- `tmux a` : S'attache à la session utilisée la plus récemment
- `<Ctrl b><d>` : se détache de la session actuelle (dans une session tmux)
- `<Ctrl b><c>` : crée une nouvelle fenêtre (dans une session tmux)
- `<Ctrl b><w>` : bascule entre les sessions et les fenêtres (dans une session tmux)
- `tmux kill-session -t nom` : détruit une session nommée

## Links
- More information: https://github.com/tmux/tmux
