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

open Syntax
module Int_dict = Map.Make (struct type t = int let compare = compare end)

let varnum = ref 0
let newvar () =
  let ret = !varnum in
  incr varnum;
  ret

let rec relambda program =
  List.map relambda_each program
and relambda_each = function
  | Definition (funname, expr) ->
     Definition (funname, h expr)
and h = function
  | Apply (Ident B, [x; y; z]) -> 
     (* B \x y z ==> x (y z) *)
     Apply (h x, [Apply (h y, [h z])])
  | Apply (Ident B, [x; y]) ->
     (* B \x y ==> \z. x (y z) *)
     let z = newvar() in
     let zexpr = Arg z in
     Lambda (z, Apply (h x, [Apply (h y, [zexpr])]))
  | Apply (Ident B, [x]) ->
     (* B \x ==> \y.\z. x (y z) *)
     let y = newvar() in
     let yexpr = Arg y in
     let z = newvar() in
     let zexpr = Arg z in
     Lambda (y, Lambda(z, Apply (h x, [Apply (yexpr, [zexpr])])))
  | Apply (Ident C, [f; x; y]) ->
     (* C f x y ==> f y x *)
     Apply (h f, [h y; h x])
  | Apply (Ident C, [f; x]) ->
     (* C f x ==> \y -> f y x *)
     let y = newvar () in
     let yexpr = Arg y in
     Lambda (y, Apply (h f, [yexpr; h x]))
  | Apply (Ident C, [f]) ->
     (* C f ==> \x.\y. f y x *)
     let x = newvar () in
     let xexpr = Arg x in
     let y = newvar () in
     let yexpr = Arg y in
     Lambda (x, Lambda (y, Apply (h f, [yexpr; xexpr])))
  | Apply (Ident S, [f; g; x]) ->
     (* S f g x = (f x) (g x) <=> \z.((f z) (g z)) x*)
     let z = newvar() in
     let zexpr = Arg z in
     let combined =
       let fexpr = Apply ((h f), [zexpr]) in
       let gexpr = Apply ((h g), [zexpr]) in
       Apply (fexpr, [gexpr])
     in
     Apply (Lambda (z, combined), [h x])
  | Apply (Ident S, [f; g]) ->
     (* S f g = \x. S f g x = (f x) (g x) <=> \z.((f z) (g z)) *)
     let z = newvar() in
     let zexpr = Arg z in
     let combined =
       let fexpr = Apply ((h f), [zexpr]) in
       let gexpr = Apply ((h g), [zexpr]) in
       Apply (fexpr, [gexpr])
     in
     Lambda (z, combined)
  | Apply (Ident S, [f]) ->
     (* S f = \g.\x. S f g x = \g.\x.(f x) (g x) <=> \g.\z.((f z) (g z)) *)
     let g = newvar() in
     let gexpr = Arg g in
     let z = newvar() in
     let zexpr = Arg z in
     let combined =
       let fexpr = Apply ((h f), [zexpr]) in
       let gexpr = Apply (gexpr, [zexpr]) in
       Apply (fexpr, [gexpr])
     in
     Lambda (g, Lambda (z, combined))
  | Apply (Ident K, [x; _y]) ->
     (* K x y -> x *)
     h x
  | Apply (Ident I, [body]) -> h body (* I x = x *)
  | Apply (Ident Cons, body) ->
     (
       try List (cons_helper body)
       with
         Exit ->
          Apply (Ident Cons, List.map h body)
     )
  | Apply (Ident Neg, [Num n]) -> Num (Z.neg n)
  | Apply (f, body) -> Apply (h f, List.map h body)
  | Ident t -> Ident t
  | Num n -> Num n
  | List xs -> List xs
  | Arg x -> Arg x
  | Lambda (v, e) -> Lambda (v, h e)
and cons_helper = function
  | [x; Ident Nil] -> [h x]
  | [x; Apply (Ident Cons, body)] ->
     (h x) :: cons_helper body
  | _ -> raise Exit
