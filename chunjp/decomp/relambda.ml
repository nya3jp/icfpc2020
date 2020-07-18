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
     (* B \x y z -> x (y z) *)
     Apply (h x, [Apply (h y, [h z])])
  | Apply (Ident B, [x; y]) ->
     (* B \x y -> \z -> x (y z) *)
     let z = newvar() in
     let zexpr = Arg z in
     Lambda ([z], Apply (h x, [Apply (h y, [zexpr])]))
  | Apply (Ident C, [f; x; y]) ->
     (* C f x y -> f y x *)
     Apply (h f, [h y; h x])
  | Apply (Ident C, [f; x]) ->
     (* C f x -> \y -> f y x *)
     let y = newvar () in
     let yexpr = Arg y in
     Lambda ([y], Apply (h f, [yexpr; h x]))
  | Apply (Ident S, [f; g; x]) ->
     (* S f g x = (f x) (g x) <=> \z.((f z) (g z)) x*)
     let z = newvar() in
     let zexpr = Arg z in
     let combined =
       let fexpr = Apply ((h f), [zexpr]) in
       let gexpr = Apply ((h g), [zexpr]) in
       Apply (fexpr, [gexpr])
     in
     Apply (Lambda ([z], combined), [h x])
  | Apply (Ident S, [f; g]) ->
     (* S f g = \x. S f g x = (f x) (g x) <=> \z.((f z) (g z)) *)
     let z = newvar() in
     let zexpr = Arg z in
     let combined =
       let fexpr = Apply ((h f), [zexpr]) in
       let gexpr = Apply ((h g), [zexpr]) in
       Apply (fexpr, [gexpr])
     in
     Lambda ([z], combined)
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
  | List _ -> failwith "Syntax.List: should not appear in Relambda.h"
  | Arg _ -> failwith "Syntax.Arg: should not appear in Relambda.h"
  | Lambda _ -> failwith "Syntax.Lambda: should not appear in Relambda.h"
and cons_helper = function
  | [x; Ident Nil] -> [h x]
  | [x; Apply (Ident Cons, body)] ->
     (h x) :: cons_helper body
  | _ -> raise Exit
