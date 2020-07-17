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
