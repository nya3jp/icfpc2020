// Binary dashboard-server is an HTTP server for the dashboard.
package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
)

func main() {
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
