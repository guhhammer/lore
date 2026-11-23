# slmgr.vbs

Instala, activa y gestiona licencias de Windows. Este comando puede sobrescribir, desactivar y/o eliminar la licencia actual de Windows. Proceda con precaución.

## Usage
- `cscript slmgr.vbs /dli` : muestrad la información actual de la licencia de Windows
- `cscript slmgr.vbs /dti` : muestrad el iD de instalación del dispositivo actual. Útil para activación de licencia sin conexión
- `cscript slmgr.vbs /xpr` : muestra la fecha y hora de caducidad e[xp]iración de la licencia actual
- `cscript slmgr.vbs /ipk product_key` : instala una nueva clavek de producto de licencia de Windows. Requiere privilegios de Administrador y sobrescribirá la licencia existente
- `cscript slmgr.vbs /ato` : activa la licencia de producto de Windows online. Requiere privilegios de Administrador
- `cscript slmgr.vbs /atp confirmation_id` : activa la licencia de producto de Windows offline (sin conexión). Requiere privilegios de Administrador y un ID de Confirmación proporcionado por el Centro de Activación de Productos Microsoft
- `cscript slmgr.vbs /cpky` : quita c la clave [ky] de un producto actual del registro de Windows. No desactiva ni desinstala la licencia, pero previene que la clave sea robada por programas maliciosos
- `cscript slmgr.vbs /upk` : desinstalau la licencia actual (por su clavek de producto)
