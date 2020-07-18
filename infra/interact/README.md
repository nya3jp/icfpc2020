# interact.py

A small python program that interacts with the contest server and the AI
programs.

This program takes three arguments: the server URL, a player key, and an AI
program executable. It invokes the AI program with a player key as an argument
and read a line from it's output. Every time the AI program writes out a string,
it sends it to the server endpoint, gets the response, and writes the response
back to the AI program.

- run.sh, testclient.go, testserver.go:

  Test programs for interact.py. Run `go run testserver.go`, and it starts an
  HTTP server at http://localhost:8080. Compile `go build testclient.go`. Then
  run `bash run.sh http://localhost:8080 12345". The first argument is the
  server URL and the second argument is the player key (== random integer).
