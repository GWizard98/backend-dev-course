package main

import (
	"log"
	"net/http"

	"example.com/backend-dev-course/module02/go/internal/server"
)

func main() {
	r := server.NewRouter()
	addr := ":3000"
	log.Printf("listening on %s", addr)
	log.Fatal(http.ListenAndServe(addr, r))
}
