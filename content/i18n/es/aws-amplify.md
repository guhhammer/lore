# aws amplify

Plataforma de desarrollo para crear aplicaciones móviles y web seguras y escalables.

## Usage
- `aws amplify create-app --name nombre_de_la_app --description descripción --repository url_repositorio --platform plataforma --environment-variables variables_de_entorno --tags etiquetas` : crea una nueva aplicación Amplify
- `aws amplify delete-app --app-id id_aplicación` : elimina una aplicación Amplify existente
- `aws amplify get-app --app-id id_aplicación` : obtiene detalles de una aplicación Amplify determinada
- `aws amplify list-apps` : lista todas las aplicaciones de Amplify
- `aws amplify update-app --app-id id_aplicación --name nuevo_nombre --description nueva_descripción --repository nuevo_url_repositorio --environment-variables variables_nuevo_entorno --tags nuevas_etiquetas` : actualiza la configuración de una aplicación Amplify
- `aws amplify create-backend-environment --app-id id_aplicación --environment-name nombre_del_entorno --deployment-artifacts artefactos` : añade un nuevo entorno backend a una aplicación Amplify
- `aws amplify delete-backend-environment --app-id id_aplicación --environment-name nombre_del_entorno` : elimina un entorno backend de una aplicación Amplify
- `aws amplify list-backend-environments --app-id id_aplicación` : lista todos los entornos backend de una aplicación Amplify

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/amplify/
