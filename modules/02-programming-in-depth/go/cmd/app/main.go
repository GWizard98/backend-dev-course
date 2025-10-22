package main

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/go-chi/chi/v5"
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

func main() {
	r := chi.NewRouter()
	r.Post("/normalize", normalizeHandler)
	addr := ":3000"
	log.Printf("listening on %s", addr)
	log.Fatal(http.ListenAndServe(addr, r))
}
