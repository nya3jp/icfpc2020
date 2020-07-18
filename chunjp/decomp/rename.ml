open Syntax
module Int_dict = Map.Make (struct type t = int let compare = compare end)

let rec rename program =
  List.map rename_each program
and rename_each = function
  | Definition (funname, expr) ->
     Definition (funname, clean expr)
and clean = function
    Apply (Lambda ([oldv], e), [Arg newv]) ->
     (clean (replace_var oldv newv e))
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
    | Lambda (args, v) -> Lambda (args, iter v)
    | Ident i -> Ident i
    | Num n -> Num n
    | List xs -> List (List.map iter xs)
  in
  iter
