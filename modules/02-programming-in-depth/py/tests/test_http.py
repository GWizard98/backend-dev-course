from fastapi.testclient import TestClient
from app.main import app

def test_normalize_400_on_empty():
    client = TestClient(app)
    r = client.post("/normalize", json={"input": "   "})
    assert r.status_code == 400
    assert r.headers.get("x-request-id")


def test_normalize_200_ok():
    client = TestClient(app)
    r = client.post("/normalize", json={"input": " Foo\tBar\nBaz "})
    assert r.status_code == 200
    assert r.headers.get("x-request-id")
    assert r.json() == {"normalized": "Foo Bar Baz"}
