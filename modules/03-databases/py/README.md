# Track: Python (SQLAlchemy + Alembic)

Recommended: PostgreSQL with SQLAlchemy Core/ORM and Alembic for migrations.

Setup
- pip install sqlalchemy psycopg[binary] alembic
- alembic init migrations (edit alembic.ini with sqlalchemy.url or use env var)
- Set DATABASE_URL=postgresql+psycopg://user:pass@localhost:5432/bdcourse
- alembic revision -m "init" && alembic upgrade head

Local development
- Lint: ruff check . && black --check .
- Test all: pytest -q
- Run app/service: uvicorn app.main:app --reload (ensure DB env set)
