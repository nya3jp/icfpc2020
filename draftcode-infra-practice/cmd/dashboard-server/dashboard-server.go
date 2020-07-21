// Copyright 2020 Google LLC
// Copyright 2020 Team Spacecat
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
