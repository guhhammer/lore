# eventcreate

Crea entradas personalizadas en el registro de eventos. Los ID de evento pueden ser cualquier número entre 1 y 1000.

## Usage
- `eventcreate /t success|error|warning|information /id 1..1000 /d "mensaje"` : crea un nuevo evento con un ID dado (1-1000) en el registro
- `eventcreate /l nombre_del_registro /t tipo /id id /d "mensaje"` : crea un evento en un registro de eventos específico
- `eventcreate /so nombre_de_la_fuente /t tipo /id id /d "mensaje"` : crea un evento con una fuente específica
- `eventcreate /s nombre_del_host /u usuario /p contraseña /t tipo /id id /d "mensaje"` : crea un evento en el registro de eventos de una máquina remota
