package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/nya3jp/icfpc2020/draftcode/interpreter"
)

func main() {
	if len(os.Args) != 3 {
		os.Exit(1)
	}

	fmt.Println("(load \"./prelude.scm\")")
	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		ss := strings.Split(scanner.Text(), " = ")
		if len(ss) != 2 {
			log.Fatal("not len 2")
		}
		expr, err := interpreter.Parse(ss[1])
		if err != nil {
			log.Fatal(err)
		}
		name := ss[0]
		if ss[0][0] == ':' {
			name = "def" + ss[0][1:]
		}
		fmt.Printf("(define (%s) %s)\n", name, expr.ToSExp())
	}
	expr, err := interpreter.Parse(os.Args[2])
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("(define (result) %s)\n", expr.ToSExp())
	fmt.Println("(printseq (result))")
}
