# runcon

Ejecuta un programa en un contexto de seguridad de SELinux diferente.

## Usage
- `runcon` : muestra el contexto de seguridad del contexto de ejecución actual
- `runcon -t dominio_t comando` : especifica el dominio en el que se ejecutará un comando
- `runcon -r rol_r comando` : especifica el rol del contexto con el que se ejecutará un comando
- `runcon usuario_u:rol_r:dominio_t comando` : especifica el contexto completo con el que ejecutar un comando
