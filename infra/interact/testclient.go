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
