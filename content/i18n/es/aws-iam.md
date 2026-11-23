# aws iam

Interactúa con el Manejo de Identidad y Acceso (o "IAM" en inglés), un servicio web para controlar seguramente el acceso a servicios de AWS.

## Usage
- `aws iam list-users` : lista usuarios
- `aws iam list-policies` : lista políticas
- `aws iam list-groups` : lista grupos
- `aws iam get-group --group-name nombre_del_grupo` : obtiene los usuarios en un grupo
- `aws iam get-policy --policy-arn arn:aws:iam::aws:policy/nombre_de_política` : describe una política IAM
- `aws iam list-access-keys` : lista claves de acceso
- `aws iam list-access-keys --user-name usuario` : lista claves de acceso para un usuario específico
- `aws iam help` : muestra la ayuda

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/iam/
