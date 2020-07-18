type t =
  Named of string
| B
| C
| S
| K
| I
| Add
| Sub
| Mul
| Div
| Neg
| Lt
| Eq
| Cons
| Nil
| Isnil
| Car
| Cdr

let show = function
  | Named s -> s
  | B -> "b"
  | C -> "c"
  | S -> "s"
  | K -> "k"
  | I -> "i"
  | Add -> "+"
  | Sub -> "-"
  | Mul -> "*"
  | Div -> "/"
  | Neg -> "~-"
  | Lt -> "<"
  | Eq -> "=="
  | Cons -> "cons"
  | Nil -> "nil"
  | Isnil -> "isnil"
  | Car -> "car"
  | Cdr -> "cdr"

let pp fmt x =
  Format.pp_print_string fmt (show x)
