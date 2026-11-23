# socat

Relé polivalente (SOcket CAT). Más información: <http://www.dest-unreach.org/socat/>.

## Usage
- `sudo socat - TCP-LISTEN:8080,fork` : escucha un puerto, espera una conexión entrante y transfiere datos a STDIO
- `sudo socat OPENSSL-LISTEN:4433,reuseaddr,cert=./cert.pem,cafile=./ca.cert.pem,key=./key.pem,verify=0 STDOUT` : escucha en un puerto usando SSL e imprime a `stdout`
- `sudo socat - TCP4:www.example.com:80` : crea una conexión a un host y puerto, transfiere datos en STDIO al host conectado
- `sudo socat TCP-LISTEN:80,fork TCP4:www.example.com:80` : reenvía los datos entrantes de un puerto local a otro host y puerto
- `echo "Hello Multicast" | socat - UDP4-DATAGRAM:224.0.0.1:5000` : envía datos con un esquema de enrutamiento multicast
- `socat - UDP4-RECVFROM:5000` : recibe datos de un multicast
