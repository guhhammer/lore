# gh attestation

Descarga y verifica las certificaciones de los artefactos para garantizar su integridad y procedencia.

## Usage
- `gh at download ruta/al/artefacto.bin -R propietario/repositorio` : descarga certificaciones para un archivo local asociado a un repositorio específico
- `gh at download oci://imagen_uri -o nombre_de_la_organización` : descarga certificaciones para una imagen de contenedor OCI asociada a una organización
- `gh at verify ruta/al/artefacto.bin -R propietario/repositorio` : verifica un artefacto local en línea con respecto a las certificaciones de un repositorio específico
- `gh at verify ruta/al/artefacto.bin -o nombre_de_la_organización --signer-workflow propietario/repositorio/ruta/a/workflow.yml` : verifica un artefacto, requiriendo que haya sido firmado por un flujo de trabajo reutilizable específico para una mayor seguridad
- `gh at verify ruta/al/artefacto.bin -o nombre_de_la_organización --format json` : verifica un artefacto y generar los resultados detallados de la verificación en formato JSON para su uso en motores de políticas
- `gh at verify ruta/al/artefacto.bin -b ruta/a/bundle.jsonl --custom-trusted-root ruta/a/trusted_root.jsonl` : realiza una verificación totalmente sin conexión utilizando un paquete descargado y un archivo raíz de confianza personalizado
- `gh at trusted-root >ruta/a/trusted_root.jsonl` : guarda la raíz de confianza de los certificados de firma en un archivo para la verificación sin conexión

## Links
- More information: https://cli.github.com/manual/gh_attestation
