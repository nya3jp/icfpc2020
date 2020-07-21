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

type ident = string
[@@deriving show]

type t =
  Apply of t * t list
| Ident of Ident.t
| Num of Z.t [@printer fun fmt -> Z.pp_print fmt ]
| Lambda of int * t
| Arg of int
| List of t list
[@@deriving show]

type definition =
  Definition of Ident.t * t
[@@deriving show]

type definitions =
  definition list
[@@deriving show]



module M = Map.Make (struct type t = int let compare = compare end)

let global_rename_table : ((string, string) Hashtbl.t) = Hashtbl.create 10

let varcounter = ref 0
let to_var x =
  if x < 6 then String.make 1 ("xyzuvw".[x])
  else Printf.sprintf "x%d" x

let add_var dict id =
  if M.mem id dict
  then
    dict
  else
    let newvarname = to_var !varcounter in
    incr varcounter;
    M.add id newvarname dict

let print_ident_str id =
  if Hashtbl.mem global_rename_table id then
    Hashtbl.find global_rename_table id
  else
    id

let print_id = function
    Ident.Named id ->
     print_ident_str id
  | x -> Ident.show x 

let rec print_definition ppf = function
    Definition (id, Lambda (x, e) ) ->
     let dict = M.empty in
     let dict = add_var dict x in
     Format.fprintf ppf "%s(%a) = %a"
       (print_id id)
       (print_arg dict) x
       (print_expr_withargs dict) e
  | Definition (id, e) ->
     Format.fprintf ppf "%s = %a"
       (print_id id)
       print_expr e
and print_expr_withargs dict ppf = function
  | Apply (e, xs) ->
     Format.fprintf ppf "(%a %a)"
       (print_expr_withargs dict) e
       (Format.pp_print_list
          ~pp_sep:(fun fmt () -> Format.pp_print_string fmt " ")
          (print_expr_withargs dict)) xs
  | Arg x ->
     if M.mem x dict then
       Format.fprintf ppf "%s" (M.find x dict)
     else
       ()
  | Lambda (arg, v) ->
     let dict = add_var dict arg in
     Format.fprintf ppf "{\\%a.%a}"
       (print_arg dict) arg
       (print_expr_withargs dict) v
  | Ident (Named x) ->
     Format.fprintf ppf "%s" (print_ident_str x)
  | Ident i ->
     Format.fprintf ppf "%s" (Ident.show i)
  | Num n ->
     Z.pp_print ppf n
  | List xs ->
     Format.fprintf ppf "[%a]"
       (fun newppf listxs ->
         Format.pp_print_list
           ~pp_sep:(fun fmt () -> Format.pp_print_string fmt "; ")
           (print_expr_withargs dict) newppf listxs) xs
and print_args dict ppf args =
  let ns = List.map (fun id -> M.find id dict) args in
  Format.pp_print_list
    ~pp_sep:(fun fmt () -> Format.pp_print_string fmt ",")
    (Format.pp_print_string) ppf ns
and print_arg dict ppf arg =
  let n = M.find arg dict in
  Format.pp_print_string ppf n
and print_expr e =
  print_expr_withargs M.empty e


let print_definitions ppf defs =
  List.iter (fun def ->
      varcounter := 0;
      Format.fprintf ppf "%a\n" print_definition def) defs
