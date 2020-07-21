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

