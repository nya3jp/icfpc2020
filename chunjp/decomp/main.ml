
let () =
  let deffile = Sys.argv.(1) in
  let ch = open_in deffile in
  let programs = Parser.program Lexer.token (Lexing.from_channel ch) in
  (*Printf.printf "%s" (Syntax.show_definitions programs) *)
  let programs = Relambda.relambda programs in
  let programs = Rename.rename programs in 
  Printf.printf "%s\n" (Syntax.show_definitions programs);
  ()
