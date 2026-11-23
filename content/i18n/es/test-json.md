# test-json

Verifica si una cadena es un documento JSON válido. Nota: Este comando solo se puede usar en PowerShell.

## Usage
- `'cadena' | Test-Json` : verifica si una cadena desde `stdin` está en formato JSON
- `Test-Json -Json 'json_a_verificar'` : verifica si una cadena está en formato JSON
- `'cadena' | Test-Json -SchemaFile ruta\a\archivo_esquema.json` : verifica si una cadena desde `stdin` cumple con un esquema JSON específico
