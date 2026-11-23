# octez-client

Interactúa con la blockchain de Tezos.

## Usage
- `octez-client -E endpoint config update` : configura el cliente con una conexión a un nodo RPC de Tezos como <https://rpc.ghostnet.teztnets.com>
- `octez-client gen keys alias` : crea una cuenta y le asigna un alias local
- `octez-client get balance for alias_o_dirección` : obtiene el saldo de una cuenta por alias o dirección
- `octez-client transfer 5 from alias|address to alias|address` : transfiere a otra cuenta
- `octez-client originate contract alias transferring 0 from alias|address running ruta/al/archivo_de_origen.tz --init "almacenamiento_inicial" --burn_cap 1` : crea (despliega) un contrato inteligente, le asigna un alias local y establece su almacenamiento inicial como un valor codificado por Michelson
- `octez-client transfer 0 from alias|address to contract --entrypoint "entrypoint" --arg "parámetro" --burn-cap 1` : llama a un contrato inteligente por su alias o dirección y pasa un parámetro codificado por Michelson
- `octez-client man` : muestra la ayuda

## Links
- More information: https://tezos.gitlab.io/introduction/howtouse.html#client
