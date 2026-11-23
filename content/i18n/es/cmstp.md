# cmstp

Administra perfiles de servicio de conexión.

## Usage
- `cmstp "ruta\al\archivo_de_perfil"` : instala un perfil específico
- `cmstp /ns "ruta\al\archivo_de_perfil"` : instala sin crear un acceso directo en el escritorio
- `cmstp /nf "ruta\al\archivo_de_perfil"` : instala sin verificar dependencias
- `cmstp /su "ruta\al\archivo_de_perfil"` : instala solo para el usuario actual
- `cmstp /au "ruta\al\archivo_de_perfil"` : instala para todos los usuarios (requiere privilegios de administrador)
- `cmstp /s "ruta\al\archivo_de_perfil"` : instala silenciosamente sin ningún aviso
- `cmstp /u "ruta\al\archivo_de_perfil"` : desinstala un perfil específico
- `cmstp /u /s "ruta\al\archivo_de_perfil"` : desinstala silenciosamente sin un aviso de confirmación
