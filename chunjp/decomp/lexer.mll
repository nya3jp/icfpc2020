{
open Parser

let kwd_table = Hashtbl.create 10
let _ =
  List.iter (fun (kwd, tok) -> Hashtbl.add kwd_table kwd tok)
            (let open Ident in 
              [ "b", B;
	        "c", C;
              	"s", S;
		"k", K;
		"i", I;
		"add", Add;
	    	"sub", Sub;
	    	"mul", Mul;
	      	"div", Div;
		"neg", Neg;
		"lt", Lt;		
		"eq", Eq;
	      	"cons", Cons;
	      	"nil", Nil;
	      	"isnil", Isnil;
		"car", Car;
		"cdr", Cdr]) 
let id_of_string query =
    try
	Hashtbl.find kwd_table query
    with
    	Not_found -> Ident.Named query
}


let space = [' ' '\t']
let digit = ['0'-'9']
let lower = ['a'-'z']

rule token = parse
| space+
  { token lexbuf }
| "\n"
  { LF }
| ":" digit+
  { IDENT (Ident.Named (Lexing.lexeme lexbuf)) }
| "-"? digit+
  { NUM (Z.of_string (Lexing.lexeme lexbuf)) }
| lower+
  { IDENT (id_of_string (Lexing.lexeme lexbuf)) }
| "("
  { LPAREN }
| ")"
  { RPAREN }
| "="
  { EQ }
| eof
  { EOF }

{
}

