# alembic

Database migration tool for SQLAlchemy.

## Usage
- `alembic init path/to/directory` : initialize Alembic in a project
- `alembic revision --autogenerate -m "message"` : create a new migration script with autogeneration
- `alembic upgrade head` : upgrade the database to the latest revision
- `alembic downgrade -1` : downgrade the database by one revision
- `alembic history` : show the migration history

## Categories
databases

## Links
- More information: https://manned.org/alembic
- tldr page: https://tldr.inbrowser.app/pages/common/alembic
