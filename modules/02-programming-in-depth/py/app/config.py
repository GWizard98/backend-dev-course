import os
from dataclasses import dataclass

@dataclass
class Settings:
    port: int = 3000
    env: str = "dev"
    log_level: str = "info"

def load_settings() -> Settings:
    port = int(os.environ.get("PORT", 3000))
    env = os.environ.get("APP_ENV", "dev")
    log_level = os.environ.get("LOG_LEVEL", "info")
    return Settings(port=port, env=env, log_level=log_level)
