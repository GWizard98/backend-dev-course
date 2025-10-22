# Track: Python

Suggested stack: FastAPI or Flask. Testing: pytest. Lint/format: ruff/black. Logging: stdlib logging or structlog.

## Local development
- Build: n/a
- Lint: ruff check . && black --check .
- Test all: pytest -q
- Test single file/case: pytest -q path/to/test_file.py::TestClass::test_name
- Run app/service: uvicorn app:app --reload (FastAPI) or python app.py (Flask)

## Labs
Follow the shared specs in ../LABS.md. Provide FastAPI/Flask implementations with pytest.
