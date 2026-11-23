# terraform plan

Erzeugen und Anzeigen von Terraform-Ausführungsplänen.

## Usage
- `terraform plan` : erzeugen und Anzeigen des Ausführungsplans im aktuellen Verzeichnis
- `terraform plan -destroy` : einen Plan zur Zerstörung aller derzeit existierenden entfernten Objekte anzeigen
- `terraform plan -refresh-only` : anzeigen eines Plans zur Aktualisierung des Terraform-Status und der Ausgabewerte
- `terraform plan -var 'name1=wert1' -var 'name2=wert2'` : werte für Eingabevariablen festlegen
- `terraform plan -target resource_type.resource_name[index]` : anzeigen eines Plans auf eine Teilmenge von Ressourcen
- `terraform plan -json` : ausgabe eines Plans als JSON
- `terraform plan -no-color >pfad/zu/datei` : ausgabe eines Plans in eine separate Datei

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/plan
