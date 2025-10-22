import re

def normalize_user_input(s: str) -> str:
    """Trim and collapse all whitespace sequences to a single space."""
    return re.sub(r"\s+", " ", s).strip()
