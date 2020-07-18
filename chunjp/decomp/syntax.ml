type ident = string
[@@deriving show]

type t =
  Apply of t * t list
| Ident of Ident.t
| Num of Z.t [@printer fun fmt -> Z.pp_print fmt ]
| Lambda of int list * t
| Arg of int
| List of t list
[@@deriving show]

type definition =
  Definition of Ident.t * t
[@@deriving show]

type definitions =
  definition list
[@@deriving show]



