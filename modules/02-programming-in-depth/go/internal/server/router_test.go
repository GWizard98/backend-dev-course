package server

import (
	"net/http/httptest"
	"strings"
	"testing"
)

func TestNormalizeEndpoint(t *testing.T) {
	r := NewRouter()

	// 400 on empty
	req := httptest.NewRequest("POST", "/normalize", strings.NewReader(`{"input":"   "}`))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()
	r.ServeHTTP(w, req)
	if w.Code != 400 {
		t.Fatalf("expected 400, got %d", w.Code)
	}

	// 200 OK
	req2 := httptest.NewRequest("POST", "/normalize", strings.NewReader(`{"input":" Foo\tBar\nBaz "}`))
	req2.Header.Set("Content-Type", "application/json")
	w2 := httptest.NewRecorder()
	r.ServeHTTP(w2, req2)
	if w2.Code != 200 {
		t.Fatalf("expected 200, got %d", w2.Code)
	}
	if w2.Header().Get("X-Request-ID") == "" {
		t.Fatalf("expected X-Request-ID header")
	}
	if !strings.Contains(w2.Body.String(), "Foo Bar Baz") {
		t.Fatalf("expected normalized body, got %s", w2.Body.String())
	}
}
