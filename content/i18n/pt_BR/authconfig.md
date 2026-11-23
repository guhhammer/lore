# authconfig

Interface de linha comandos para configurar o sistema de autenticação.

## Usage
- `authconfig --test` : exibe as configurações atuais (ou dry run)
- `authconfig --update --passalgo=algoritmo` : configura o servidor para utilizar diferentes algoritmos de hash para as senhas
- `authconfig --update --enableldapauth` : habilita a autenticação via LDAP
- `authconfig --update --disableldapauth` : desabilita a autenticação via LDAP
- `authconfig --update --enablenis` : habilita o Network Information Service (NIS)
- `authconfig --update --enablekrb5` : habilita Kerberos
- `authconfig --update --enablewinbindauth` : habilita a autenticação Winbind (Active Directory)
- `authconfig --update --enablelocauthorize` : habilita a autorização local

## Links
- More information: https://manned.org/authconfig
