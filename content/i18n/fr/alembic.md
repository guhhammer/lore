# alembic

Outil de migration de base de données pour SQLAlchemy.

## Usage
- `alembic init chemin/vers/répertoire` : initialise Alembic dans un projet
- `alembic revision --autogenerate -m "message"` : crée un script de migration avec génération automatique
- `alembic upgrade head` : met à jour la base de données vers la dernière révision
- `alembic downgrade -1` : revient à la révision précédente
- `alembic history` : affiche l'historique des migrations

## Links
- More information: https://manned.org/alembic
