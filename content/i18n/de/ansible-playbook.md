# ansible-playbook

In Playbook definierte Aufgaben auf entfernten Rechnern über SSH ausführen.

## Usage
- `ansible-playbook playbook` : führe Aufgaben im Playbook aus
- `ansible-playbook playbook -i inventory_datei` : führe Aufgaben im Playbook mit benutzerdefiniertem Host-Bestand aus
- `ansible-playbook playbook -e "variable1=wert1 variable2=wert2"` : führe Aufgaben im Playbook aus, wobei zusätzliche Variablen über die Befehlszeile definiert werden
- `ansible-playbook playbook -e "@variablen.json"` : führe Aufgaben in Playbook mit zusätzlichen Variablen aus, die in einer JSON-Datei definiert sind
- `ansible-playbook playbook -t tag1,tag2` : führe Aufgaben im Playbook für die angegebenen Tags aus
- `ansible-playbook playbook --start-at aufgabenname` : führe Aufgaben in einem Playbook aus, die mit einer bestimmten Aufgabe beginnen

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-playbook.html
