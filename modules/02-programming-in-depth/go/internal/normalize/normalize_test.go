package normalize

import "testing"

func TestNormalizeUserInput(t *testing.T) {
	if got := NormalizeUserInput("  Hello   world\n"); got != "Hello world" {
		t.Fatalf("expected 'Hello world', got %q", got)
	}
	if got := NormalizeUserInput(""); got != "" {
		t.Fatalf("expected empty, got %q", got)
	}
	if got := NormalizeUserInput("   \t\n"); got != "" {
		t.Fatalf("expected empty after all whitespace, got %q", got)
	}
	if got := NormalizeUserInput("Foo\tBar\nBaz"); got != "Foo Bar Baz" {
		t.Fatalf("expected 'Foo Bar Baz', got %q", got)
	}
}
