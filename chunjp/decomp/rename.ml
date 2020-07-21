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

let rec rename program =
  List.map rename_each program
and rename_each = function
  | Definition (funname, expr) ->
     Definition (funname, clean expr)
and clean = function
  | Apply (Lambda (oldv, e), [Arg newv]) ->
     clean (replace_var oldv newv e)
  | Apply (Lambda (oldv, e), (Arg newv :: xs)) ->
     clean (Apply ((replace_var oldv newv e), xs))
  | Apply (f, xs) ->
     Apply (clean f, List.map clean xs)
  | Lambda (args, v) -> Lambda (args, clean v)
  | x -> x
and replace_var oldv newv =
  let rec iter = 
    function
    | Apply (e, xs) ->
       Apply (iter e, List.map iter xs)
    | Arg x when (x == oldv) ->
       Arg newv
    | Arg x -> Arg x
    | Lambda (arg, v) -> Lambda (arg, iter v)
    | Ident i -> Ident i
    | Num n -> Num n
    | List xs -> List (List.map iter xs)
  in
  iter
