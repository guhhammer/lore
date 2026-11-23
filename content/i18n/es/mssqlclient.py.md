# mssqlclient.py

Se conecta a instancias de Microsoft SQL Server y ejecuta consultas. Forma parte del paquete Impacket.

## Usage
- `mssqlclient.py -windows-auth dominio/nombre de usuario:contraseña@destino` : se conecta a un servidor MSSQL mediante autenticación de Windows
- `mssqlclient.py nombre de usuario:contraseña@destino` : se conecta mediante autenticación de SQL Server
- `mssqlclient.py dominio/nombre de usuario@destino -hashes LM_Hash:NT_Hash` : se conecta mediante autenticación "pass-the-hash"
- `mssqlclient.py -k dominio/nombre_de_usuario@destino` : se conecta mediante autenticación Kerberos (requiere tickets válidos)
- `mssqlclient.py nombre_de_usuario:contraseña@destino -query "SELECT nombre_de_usuario();"` : ejecuta un comando SQL específico al conectarse
- `mssqlclient.py nombre_de_usuario:contraseña@destino -file ruta/al/archivo_sql.sql` : ejecuta varios comandos SQL desde un archivo
- `mssqlclient.py unombre_de_usuariosername:contraseña@destino -db nombre_base_de_datos` : se conecta a una instancia de base de datos específica (el valor por defecto es `None`)
- `mssqlclient.py nombre_de_usuario:contraseña@destino -show` : muestra las consultas SQL antes de ejecutarlas

## Links
- More information: https://github.com/fortra/impacket
