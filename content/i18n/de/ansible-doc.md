# ansible-doc

Anzeigen von Informationen über die in den Ansible-Bibliotheken installierten Module. Anzeigen einer knappen Auflistung von Plugins und deren Kurzbeschreibungen.

## Usage
- `ansible-doc -l` : auflisten der verfügbaren Aktions-Plugin (Module)
- `ansible-doc -t plugin_typ -l` : auflisten der verfügbare Plugins eines bestimmten Typs
- `ansible-doc plugin_name` : anzeigen von Informationen über ein bestimmtes Aktions-Plugin (Module)
- `ansible-doc -t plugin_typ plugin_name` : anzeigen von Informationen über ein Plugin mit einem betimmten Typ
- `ansible-doc -s plugin_name` : anzeigen des Playbookausschnittes für ein Actions-Plugin (Module)
- `ansible-doc -j plugin_name` : anzeigen von Informationen über ein Aktions-Plugin (Module) als JSON

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-doc.html
