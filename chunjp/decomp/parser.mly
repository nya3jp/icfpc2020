%{
open Syntax
%}

%token LPAREN
%token RPAREN
%token <Ident.t> IDENT
%token LF
%token EQ
%token <Z.t> NUM
%token EOF

%type <Syntax.definitions> program
%start program

%%

program:
| IDENT EQ expr LF program { Definition ($1, $3) :: $5 }
| EOF { [] }

expr:
| LPAREN exprs RPAREN { Apply ((List.hd $2), (List.tl $2)) }
| IDENT { Ident $1 }
| NUM { Num $1 }

exprs:
| expr exprs { $1 :: $2 }
| { [] }

%%

