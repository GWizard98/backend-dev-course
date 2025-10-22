from app.utils import normalize_user_input

def test_trims_and_collapses_spaces():
    assert normalize_user_input("  Hello   world\n") == "Hello world"

def test_empty_and_spaces():
    assert normalize_user_input("") == ""
    assert normalize_user_input("   \t\n") == ""

def test_tabs_and_newlines():
    assert normalize_user_input("Foo\tBar\nBaz") == "Foo Bar Baz"
