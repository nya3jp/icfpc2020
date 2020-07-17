package interpreter

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

var (
	builtins = map[string]func([]Expr) (Expr, error){
		"add": BuiltinAdd,
	}
)

func BuiltinAdd(exprs []Expr) (Expr, error) {
	if len(exprs) > 2 {
		return nil, fmt.Errorf("interpreter: add cannot take more than two args")
	}
	if len(exprs) != 2 || !isNumber(exprs[0]) || !isNumber(exprs[1]) {
		return Apply{Function{"add"}, exprs}, nil
	}
	return Number{exprs[0].(Number).Value + exprs[1].(Number).Value}, nil
}

type Expr interface {
	ToSExp() string
}

type Number struct {
	Value int64
}

func (n Number) ToSExp() string {
	return fmt.Sprintf("%d", n.Value)
}

type Variable struct {
	Value int64
}

func (v Variable) ToSExp() string {
	return fmt.Sprintf("x%d", v.Value)
}

type Function struct {
	Name string
}

func (f Function) ToSExp() string {
	if f.Name == "cons" {
		return "(mycons)"
	}
	if f.Name == "cdr" {
		return "(mycdr)"
	}
	if f.Name == "car" {
		return "(mycar)"
	}
	if f.Name[0] == ':' {
		return "(def" + f.Name[1:] + ")"
	}
	return "(" + f.Name + ")"
}

type Apply struct {
	Op   Expr
	Args []Expr
}

func (a Apply) ToSExp() string {
	ss := []string{}
	for _, arg := range a.Args {
		ss = append(ss, arg.ToSExp())
	}
	return "(delay ((force " + a.Op.ToSExp() + ") " + strings.Join(ss, " ") + "))"
}

type SingleApply struct {
	Op  Expr
	Arg Expr
}

func (a SingleApply) ToSExp() string {
	return "(delay (force ((force " + a.Op.ToSExp() + ") " + a.Arg.ToSExp() + ")))"
}

func Parse(in string) (Expr, error) {
	in = regexp.MustCompile(" +").ReplaceAllString(in, " ")
	ss := strings.Split(in, " ")
	expr, left, err := parseInner(ss)
	if err != nil {
		return nil, fmt.Errorf("interpreter: parse error %v", err)
	}
	if len(left) != 0 {
		return nil, fmt.Errorf("interpreter: leftover %s", strings.Join(left, " "))
	}
	return expr, nil
}

func parseInner(ss []string) (Expr, []string, error) {
	if len(ss) == 0 {
		return nil, nil, fmt.Errorf("interpreter: end of input")
	}
	apCnt := 0
	for apCnt < len(ss) && ss[apCnt] == "ap" {
		apCnt += 1
	}
	if apCnt == len(ss) {
		return nil, nil, fmt.Errorf("interpreter: end with ap")
	}
	if apCnt > 0 {
		ss = ss[apCnt:]
		var op Expr
		if expr, left, err := parseInner(ss); err != nil {
			return nil, nil, fmt.Errorf("interpreter: parse error while parsing op: %v", err)
		} else {
			op = expr
			ss = left
		}
		var args []Expr
		for i := 0; i < apCnt; i++ {
			if expr, left, err := parseInner(ss); err != nil {
				return nil, nil, fmt.Errorf("interpreter: parse error while parsing arg: %v", err)
			} else {
				args = append(args, expr)
				ss = left
			}
		}
		res := SingleApply{op, args[0]}
		for i := 1; i < len(args); i++ {
			res = SingleApply{res, args[i]}
		}
		return res, ss, nil
	}

	tok := ss[0]
	if tok[0] == 'x' {
		v, err := strconv.ParseInt(tok[1:], 10, 64)
		if err == nil {
			return Variable{v}, ss[1:], nil
		}
	}
	if v, err := strconv.ParseInt(tok, 10, 64); err == nil {
		return Number{v}, ss[1:], nil
	}
	return Function{tok}, ss[1:], nil

}

func Interpret(e Expr) (Expr, error) {
	switch expr := e.(type) {
	case Number:
		return expr, nil
	case Variable:
		return expr, nil
	case Function:
		return expr, nil
	case Apply:
		op, err := Interpret(expr.Op)
		if err != nil {
			return nil, fmt.Errorf("interpreter: error while interpreting op: %v", err)
		}
		args := []Expr{}
		for _, argExpr := range expr.Args {
			evaledExpr, err := Interpret(argExpr)
			if err != nil {
				return nil, fmt.Errorf("interpreter: error while interpreting arg: %v", err)
			}
			args = append(args, evaledExpr)
		}

		if isFunction(op) {
			fn, ok := op.(Function)
			if !ok {
				return nil, fmt.Errorf("interpreter: cannot apply %v", op)
			}
			if builtin, ok := builtins[fn.Name]; ok {
				return builtin(args)
			}
		}
		return Apply{op, args}, nil

	default:
		return nil, fmt.Errorf("interpreter: unknown type %v", e)
	}
}

func isFunction(e Expr) bool {
	switch e.(type) {
	case Function:
		return true
	default:
		return false
	}
}

func isNumber(e Expr) bool {
	switch e.(type) {
	case Number:
		return true
	default:
		return false
	}
}
