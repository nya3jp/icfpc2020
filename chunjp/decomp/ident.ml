(**
   Copyright 2020 Google LLC
   Copyright 2020 Team Spacecat

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*)

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
