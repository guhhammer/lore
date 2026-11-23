# aws ec2

Interfaz de línea de comandos (CLI) para AWS EC2. Provee capacidad de computacion segura y redimensionable en la nube de AWS, permitiendo mayor velociddad en el desarrollo e implementación de aplicaciones.

## Usage
- `aws ec2 describe-instances --instance-ids identificador_de_instancia` : muestra información acerca de una instancia específica
- `aws ec2 describe-instances` : muestra información sobre todas las instancias
- `aws ec2 describe-volumes` : muestra información sobre todos los volúmenes EC2
- `aws ec2 delete-volume --volume-id identificador_de_volumen` : elimina un volumen EC2
- `aws ec2 create-snapshot --volume-id identificador_de_volumen` : crea una instantánea a partir de un volumen EC2
- `aws ec2 describe-images` : lista las imágenes de máquina de Amazon disponibles (AMI)
- `aws ec2 help` : lista todos los comandos EC2 disponibles
- `aws ec2 subcomando help` : muestra la ayuda para un comando EC2 específico

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ec2/
