# az devops

Administra organizaciones de Azure DevOps. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az devops login --org url_de_la_organización` : configura el Token de Acceso Personal (PAT) para iniciar sesión en una organización específica
- `az devops project show -p nombre_de_proyecto --open` : abre un proyecto en el navegador
- `az devops team list-member -p nombre_de_proyecto --team nombre_de_equipo` : lista los miembros de un equipo específico que trabaja en un proyecto en particular
- `az devops configure -l` : comprueba la configuración actual de la CLI de Azure DevOps
- `az devops configure -d project=nombre_de_proyecto organization=url_de_la_organización` : configura el comportamiento de la CLI de Azure DevOps estableciendo un proyecto predeterminado y una organización predeterminada

## Links
- More information: https://learn.microsoft.com/cli/azure/devops
