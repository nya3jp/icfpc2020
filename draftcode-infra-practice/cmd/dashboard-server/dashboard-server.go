// Binary dashboard-server is an HTTP server for the dashboard.
package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"path/filepath"
)

func main() {
	if dir, err := filepath.Abs(filepath.Dir(os.Args[0])); err != nil {
		log.Printf("Cannot obtain the static file dir: %v", err)
	} else {
		http.Handle("/static/", http.StripPrefix("/static", http.FileServer(http.Dir(filepath.Join(dir, "static")))))
	}
	http.HandleFunc("/", rootHandler)

	port := os.Getenv("PORT")
	if port == "" {
		log.Fatalf("PORT not specified")
	}
	log.Fatal(http.ListenAndServe(fmt.Sprintf(":%s", port), nil))
}

func rootHandler(w http.ResponseWriter, r *http.Request) {
	io.WriteString(w, "Hello")
}
