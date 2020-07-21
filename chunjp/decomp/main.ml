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

let init_rename_table () =
  try
    let ch = open_in "./rename_table.txt" in
    while true do
      let line = input_line ch in
      Scanf.sscanf line "%s %s"
        (fun x y -> Hashtbl.add Syntax.global_rename_table x y)
    done;
    let () = close_in ch in
    ()
  with _ -> ()

let optloop programs =
  let programs = Relambda.relambda programs in
  let programs = Rename.rename programs in 
  programs

let () =
  let () = init_rename_table () in
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
