# authconfig

Configura los recursos de autenticación del sistema.

## Usage
- `authconfig --test` : muestra la configuración actual (o realiza una simulación)
- `authconfig --update --passalgo=algoritmo` : configura el servidor para usar un algoritmo de hash de contraseñas diferente
- `authconfig --update --enableldapauth` : habilita la autenticación LDAP
- `authconfig --update --disableldapauth` : deshabilita la autenticación LDAP
- `authconfig --update --enablenis` : habilita el servicio de información de red (NIS)
- `authconfig --update --enablekrb5` : habilita Kerberos
- `authconfig --update --enablewinbindauth` : habilita la autenticación Winbind (Active Directory)
- `authconfig --update --enablelocauthorize` : habilita la autorización local

## Links
- More information: https://manned.org/authconfig
