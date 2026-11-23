# black

Formatteer Python-code automatisch.

## Usage
- `black pad/naar/bestand_of_map` : formatteer een bestand of hele map automatisch
- `black -c "code"` : formatteer de code die is opgegeven als een string
- `black --check pad/naar/bestand_of_map` : toon of een bestand of map wijzigingen zou ondergaan als deze geformatteerd zou worden
- `black --diff pad/naar/bestand_of_map` : toon wijzigingen die in een bestand of map zouden worden aangebracht zonder ze daadwerkelijk toe te passen (dry-run)
- `black -q pad/naar/bestand_of_map` : formatteer een bestand of map automatisch, waarbij foutmeldingen uitsluitend naar `stderr` worden verstuurd
- `black -S pad/naar/bestand_of_map` : formatteer een bestand of map automatisch zonder enkele aanhalingstekens te vervangen door dubbele aanhalingstekens (adoptiehulp, vermijd dit voor nieuwe projecten)
