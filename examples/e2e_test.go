package main

import (
	"bytes"
	"flag"
	"fmt"
	"io/ioutil"
	"net"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

var (
	testURL     string
	testHandler = map[string]http.Handler{}

	testBinary = flag.String("test_binary", "", "The binary to test")
)

func init() {
	listener, err := net.Listen("tcp", ":0")
	if err != nil {
		panic(err)
	}
	testURL = fmt.Sprintf("http://localhost:%d", listener.Addr().(*net.TCPAddr).Port)
	go http.Serve(listener, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if h, ok := testHandler[strings.SplitN(strings.TrimPrefix(r.URL.Path, "/"), "/", 2)[0]]; ok {
			h.ServeHTTP(w, r)
			return
		}
		http.Error(w, http.StatusText(http.StatusNotFound), http.StatusNotFound)
	}))
}

func registerHandler(name string, h http.Handler) (string, func()) {
	testHandler[name] = h
	return testURL + "/" + name, func() { delete(testHandler, name) }
}

func runBinary(url, arg string) error {
	bin := filepath.Join(os.Getenv("RUNFILES_DIR"), os.Getenv("TEST_WORKSPACE"), *testBinary)
	cmd := exec.Command(bin, url, arg)
	cmd.Stderr = os.Stderr
	cmd.Stdout = os.Stdout
	cmd.Stdin = &bytes.Buffer{}
	return cmd.Run()
}

func TestEndToEnd(t *testing.T) {
	want := "12345678"
	got := ""
	url, closeF := registerHandler(t.Name(), http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != "POST" {
			http.Error(w, http.StatusText(http.StatusMethodNotAllowed), http.StatusMethodNotAllowed)
			return
		}
		bs, err := ioutil.ReadAll(r.Body)
		if err != nil {
			t.Fatal(err)
		}
		got = string(bs)
		w.Write(bs)
	}))
	defer closeF()
	if err := runBinary(url, want); err != nil {
		t.Fatal(err)
	}

	if want != got {
		t.Errorf("want %s, got %s", want, got)
	}
}
