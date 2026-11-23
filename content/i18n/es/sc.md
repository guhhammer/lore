# sc

Comunicación con el Administrador de Control de Servicios y los servicios.

## Usage
- `sc.exe query nombre_del_servicio` : muestra el estado de un servicio (al no nombrar un servicio se listan todos los servicios)
- `sc.exe create nombre_del_servicio binpath= ruta\al\binario_del_servicio` : inicia un servicio asincrónicamente
- `sc.exe delete nombre_del_servicio` : detiene un servicio asincrónicamente
- `sc.exe config nombre_del_servicio type= tipo_de_servicio` : establece el tipo de servicio
