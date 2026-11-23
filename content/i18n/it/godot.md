# godot

Motore di gioco open source 2D e 3D.

## Usage
- `godot` : esegue un progetto se la directory corrente contiene un file `project.godot`, altrimenti apre il project manager
- `godot -e` : modifica un progetto (la directory corrente deve contenere un file `project.godot`)
- `godot -p` : apre il project manager anche se la directory corrente contiene un file `project.godot`
- `godot --export-release preset output_path` : esporta un progetto per il rilascio usando un preset di esportazione specificato (il preset deve essere definito nel progetto)
- `godot -s script.gd` : esegue un file GDScript standalone (lo script deve ereditare da `SceneTree` o `MainLoop`)
