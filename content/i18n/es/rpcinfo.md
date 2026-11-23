# rpcinfo

Lista programas vía RPC en computadoras remotas.

## Usage
- `rpcinfo` : lista todos los programas registrados en la computadora local
- `rpcinfo /p nombre_de_computadora` : lista todos los programas registrados en una computadora remota
- `rpcinfo /t nombre_de_computadora nombre_del_programa` : llama a un programa específico en una computadora remota usando TCP
- `rpcinfo /u nombre_de_computadora nombre_del_programa` : llama a un programa específico en una computadora remota usando UDP
