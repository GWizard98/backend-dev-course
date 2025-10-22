package server

import (
	"encoding/json"
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"example.com/backend-dev-course/module02/go/internal/normalize"
)

type normalizeRequest struct {
	Input string `json:"input"`
}

type normalizeResponse struct {
	Normalized string `json:"normalized"`
}

func normalizeHandler(w http.ResponseWriter, r *http.Request) {
	var req normalizeRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		w.WriteHeader(http.StatusBadRequest)
		_ = json.NewEncoder(w).Encode(map[string]string{"error": "invalid json"})
		return
	}
	in := req.Input
	if len(in) == 0 || len(normalize.NormalizeUserInput(in)) == 0 {
		w.WriteHeader(http.StatusBadRequest)
		_ = json.NewEncoder(w).Encode(map[string]string{"error": "input must not be empty"})
		return
	}
	res := normalizeResponse{Normalized: normalize.NormalizeUserInput(in)}
	w.Header().Set("Content-Type", "application/json")
	_ = json.NewEncoder(w).Encode(res)
}

func NewRouter() *chi.Mux {
	r := chi.NewRouter()
	r.Use(middleware.RequestID)
	r.Use(middleware.Logger)
	// Ensure X-Request-ID header is included in responses
	r.Use(func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			next.ServeHTTP(w, r)
			if id := middleware.GetReqID(r.Context()); id != "" {
				w.Header().Set("X-Request-ID", id)
			}
		})
	})

	r.Post("/normalize", normalizeHandler)
	return r
}
