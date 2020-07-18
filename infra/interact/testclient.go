package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	log.Print(os.Args)
	scanner := bufio.NewScanner(os.Stdin)

	testInteract(scanner, "test1")
	testInteract(scanner, "test2")
	testInteract(scanner, "test3")
}

func testInteract(scanner *bufio.Scanner, mes string) {
	fmt.Println(mes)
	if scanner.Scan() {
		log.Printf("received %q", scanner.Text())
	} else {
		log.Printf("error %v", scanner.Err())
	}
}
