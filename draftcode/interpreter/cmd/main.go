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
		fmt.Printf("%s = %s\n", ss[0], expr.ToSExp())
	}
}
