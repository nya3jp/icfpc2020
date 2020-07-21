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

package interpreter

import (
	"testing"
)

func TestIntepret(t *testing.T) {
	// tests := []struct {
	// 	input string
	// 	want  Expr
	// }{
	// 	{"ap ap add 1 2", Number{3}},
	// 	{"ap add 1", Apply{Function{"add"}, []Expr{Number{1}}}},
	// 	{"ap ap add 1 x0", Apply{Function{"add"}, []Expr{Number{1}, Variable{0}}}},
	// }
	// for _, tc := range tests {
	// 	expr, err := Parse(tc.input)
	// 	if err != nil {
	// 		t.Errorf("%s: got error: %#v", tc.input, err)
	// 		continue
	// 	}
	// 	got, err := Interpret(expr)
	// 	if !cmp.Equal(got, tc.want) {
	// 		t.Errorf("%s: got %#v, want %#v", tc.input, got, tc.want)
	// 	}
	// }
}
