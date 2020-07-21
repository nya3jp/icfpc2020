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
	"image"
	"image/color"
	"image/png"
	"io/ioutil"
	"log"
	"os"
	"strings"
)

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}

func run() error {
	b, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		return err
	}

	ss := strings.Split(string(b), "\n")
	h := len(ss)
	w := 0
	for _, s := range ss {
		if len(s) > w {
			w = len(s)
		}
	}

	img := image.NewGray(image.Rect(0, 0, w, h))

	for i := 0; i < h; i++ {
		for j := 0; j < len(ss[i]); j++ {
			var c color.Color
			if ss[i][j] == ' ' {
				c = color.Black
			} else {
				c = color.White
			}
			img.Set(j, i, c)
		}
	}

	if err := png.Encode(os.Stdout, img); err != nil {
		return err
	}
	return nil
}
