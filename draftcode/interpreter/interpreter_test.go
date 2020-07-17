package interpreter

import "testing"

func TestIntepret(t *testing.T) {
	got, err := parseAndIntepret("ap ap add 1 2")
	want := Number{3}
	if err != nil {
		t.Errorf("got error: %v", err)
	}
	if got != want {
		t.Errorf("got %v, want %v", got, want)
	}
}

func parseAndIntepret(in string) (Expr, error) {
	expr, err := Parse(in)
	if err != nil {
		return nil, err
	}
	return Interpret(expr)
}
