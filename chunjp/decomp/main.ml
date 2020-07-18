
let optloop programs =
  let programs = Relambda.relambda programs in
  let programs = Rename.rename programs in 
  programs

let () =
  let deffile = Sys.argv.(1) in
  let ch = open_in deffile in
  let programs = Parser.program Lexer.token (Lexing.from_channel ch) in
  (*Printf.printf "%s" (Syntax.show_definitions programs) *)
  let programs = ref programs in
  for _i = 0 to 10 do
    programs := optloop !programs
  done;
  (*Printf.printf "%s\n" (Syntax.show_definitions programs);*)
  Syntax.print_definitions Format.std_formatter !programs;
  ()
