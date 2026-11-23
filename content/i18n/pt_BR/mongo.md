# mongo

Cliente shell interativo de MongoDB.

## Usage
- `mongo` : conecta a uma base de dados local na porta padrão (mongodb://localhost:27017)
- `mongo --host servidor --port porta base_de_dados` : conecta a uma base de dados em um servidor e porta específicos
- `mongo --host servidor --port porta --username usuário --authenticationDatabase auth_base_de_dados base_de_dados` : autentica usando, na base de dados especificada, o nome de usuário especificado (uma senha será solicitada)
- `mongo --eval 'JSON.stringify(db.foo.findOne())' base_de_dados` : avalia JavaScript na base de dados

## Links
- More information: https://www.mongodb.com/docs/mongodb-shell/
