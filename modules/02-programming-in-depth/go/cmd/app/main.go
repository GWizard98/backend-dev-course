package main

import (
	"fmt"
	"log"
	"net/http"

	"example.com/backend-dev-course/module02/go/internal/config"
	"example.com/backend-dev-course/module02/go/internal/server"
)

func main() {
	r := server.NewRouter()
	cfg := config.Load()
	addr := fmt.Sprintf(":%d", cfg.Port)
	log.Printf("listening on %s env=%s", addr, cfg.Env)
	log.Fatal(http.ListenAndServe(addr, r))
}
