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
	fmt.Println("(load \"./prelude.scm\")")
	scanner := bufio.NewScanner(os.Stdin)
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
	expr, err := interpreter.Parse("ap ap galaxy nil ap ap cons 0 0")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("(define (result) %s)\n", expr.ToSExp())
	fmt.Println("(printseq (result))")
}
