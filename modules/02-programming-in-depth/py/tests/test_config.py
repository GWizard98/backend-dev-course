from app.config import load_settings

def test_config_defaults(monkeypatch):
    monkeypatch.delenv("PORT", raising=False)
    monkeypatch.delenv("APP_ENV", raising=False)
    monkeypatch.delenv("LOG_LEVEL", raising=False)
    s = load_settings()
    assert s.port == 3000
    assert s.env == "dev"
    assert s.log_level == "info"


def test_config_from_env(monkeypatch):
    monkeypatch.setenv("PORT", "5050")
    monkeypatch.setenv("APP_ENV", "test")
    monkeypatch.setenv("LOG_LEVEL", "debug")
    s = load_settings()
    assert s.port == 5050
    assert s.env == "test"
    assert s.log_level == "debug"
