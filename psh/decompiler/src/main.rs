// Copyright 2020 Google LLC
// Copyright 2020 Team Spacecat
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{env, fs, io};

use decompiler::parser;
use decompiler::lambda;
use decompiler::simplified;
use decompiler::expr;
use decompiler::expr::Type;


fn main() -> io::Result<()> {
    let mut env = expr::Env::new();

    // The format of picture is "[size, bits...]". For each bits has 63 bits.
    env.insert(":1029", "Bitmap.Galaxy", None);
    env.insert(":1030", "Bitmap.App", None);
    env.insert(":1031", "Bitmap.Mul", None);
    env.insert(":1032", "Bitmap.Pow2", None);
    env.insert(":1034", "Bitmap.TotalEnergy", None);  // For bomb explanation.
    env.insert(":1035", "Bitmap.TwoSquares", None);
    env.insert(":1036", "Bitmap.Define", None);
    env.insert(":1037", "Bitmap.Sum", None);
    env.insert(":1038", "Bitmap.FourDots", None);
    env.insert(":1039", "Bitmap.Bomb", None);
    env.insert(":1040", "Bitmap.Thruster", None);
    env.insert(":1041", "Bitmap.VarX0", None);  // TODO
    env.insert(":1042", "Bitmap.VarX1", None);  // TODO

    env.insert(":1043", "SparseBitmap.SymbolHuman", None);
    env.insert(":1044", "SparseBitmap.SymbolEndo", None);
    env.insert(":1045", "SparseBitmap.SymbolAlien1", None);
    env.insert(":1046", "SparseBitmap.SymbolAlien2", None);
    env.insert(":1047", "SparseBitmap.SymbolAlien3", None);
    env.insert(":1048", "SparseBitmap.SymbolAlien4", None);
    env.insert(":1049", "SparseBitmap.SymbolAlien5", None);
    env.insert(":1050", "SparseBitmap.SymbolAlien6", None);
    env.insert(":1051", "SparseBitmap.SymbolAlien7", None);
    env.insert(":1052", "SparseBitmap.SymbolAlien8", None);
    env.insert(":1053", "SparseBitmap.SymbolAlien9", None);

    env.insert(":1059", "SparseBitmap.PictureHuman", None);
    env.insert(":1060", "SparseBitmap.PictureEndo", None);
    env.insert(":1061", "SparseBitmap.PictureAlien1", None);
    env.insert(":1062", "SparseBitmap.PictureAlien2", None);
    env.insert(":1063", "SparseBitmap.PictureAlien3", None);
    env.insert(":1064", "SparseBitmap.PictureAlien4", None);
    env.insert(":1065", "SparseBitmap.PictureAlien5", None);
    env.insert(":1066", "SparseBitmap.PictureAlien6", None);
    env.insert(":1067", "SparseBitmap.PictureAlien7", None);
    env.insert(":1068", "SparseBitmap.PictureAlien8", None);
    env.insert(":1069", "SparseBitmap.PictureAlien9", None);

    env.insert(":1075", "SparseBitmap.LargeTrue", None);
    env.insert(":1076", "SparseBitmap.LargeFalse", None);
    env.insert(":1077", "SparseBitmap.LargeHeatMax", None);
    env.insert(":1078", "SparseBitmap.LargeThruster", None);

    env.insert(":1079", "Space.Aliens", None);  // ID, pos, symbol, picture, (?).

    env.insert(":1080", "Bitmap.Laser", None);
    env.insert(":1081", "Bitmap.Split", None);
    env.insert(":1082", "Bitmap.Game", None);
    env.insert(":1083", "Bitmap.Attacker1", None);
    env.insert(":1084", "Bitmap.Attacker2", None);
    env.insert(":1085", "Bitmap.Attacker3", None);
    env.insert(":1086", "Bitmap.Attacker4", None);
    env.insert(":1087", "Bitmap.Defender1", None);
    env.insert(":1088", "Bitmap.Defender2", None);
    env.insert(":1089", "Bitmap.Defender3", None);
    env.insert(":1090", "Bitmap.Defender4", None);

    env.insert(":1091", "Machine.Attackers", None);
    env.insert(":1092", "Machine.Defenders", None);

    env.insert(":1093", "Bitmap.SplitClose", None);

    env.insert(":1097", "Bitmap.Energy", None);
    env.insert(":1098", "Bitmap.LaserMax", None);
    env.insert(":1099", "Bitmap.Cooldown", None);
    env.insert(":1100", "Bitmap.Life", None);

    env.insert(":1101", "Bitmap.Heat", None);
    env.insert(":1102", "Machine.Params", None);

    env.insert(":1103", "SparseBitmap.Space1", None);
    env.insert(":1104", "SparseBitmap.Space2", None);
    env.insert(":1105", "SparseBitmap.Space3", None);

    env.insert(":1107", "Print.thurster_range", None);  // Position.
    env.insert(":1108", "Print.heat_range", None);  // position.

    env.insert(":1109", "Msg.error", None);
    env.insert(":1110", "Msg.create", None);
    env.insert(":1111", "Msg.join", None);
    env.insert(":1112", "Msg.start", None);
    env.insert(":1113", "Msg.command", None);
    env.insert(":1114", "Msg.history", None);

    // Basic library.
    env.insert(":1115", "cons", None);
    env.insert(":1116", "List.hd", None);

    // Arithmetic library.
    env.insert(":1117", "Int.pow2", None);
    env.insert(":1118", "Int.log2", None);
    env.insert(":1119", "Int.log_x_y", None);
    env.insert(":1120", "Int.abs", None);
    env.insert(":1121", "Int.max", None);
    env.insert(":1122", "Int.min", None);

    // List library.
    env.insert(":1124", "List.mem", Some(Type::Func(vec!((None, None), (None, None)), Box::new(Some(Type::Bool)))));
    env.insert(":1126", "List.map", None);
    env.insert(":1127", "List.mapi", None);
    env.insert(":1128", "List.len", None);
    env.insert(":1131", "List.concat", None);
    env.insert(":1132", "List.foldl", None);
    env.insert(":1133", "List.foldr", None);
    env.insert(":1134", "List.flatten", None);
    env.insert(":1135", "List.filter", None);
    env.insert(":1136", "List.filteri", None);
    env.insert(":1137", "List.exists", None);
    env.insert(":1138", "IntList.make_rev", None);
    env.insert(":1139", "IntList.make", None);
    env.insert(":1141", "List.nth", None);
    env.insert(":1142", "List.nth_list", None);
    env.insert(":1143", "IntList.sum", None);
    env.insert(":1144", "List.replace_nth", None);
    env.insert(":1146", "IntList.max", None);
    env.insert(":1147", "List.select", None);
    env.insert(":1149", "IntList.min", None);
    env.insert(":1150", "List.map_sort", None);
    env.insert(":1152", "List.sort", None);
    env.insert(":1153", "List.filter2", None);
    env.insert(":1155", "IntList.unique", None);

    // Geometric library.
    env.insert(":1162", "Vec2.new", None);

    env.insert(":1166", "Rect.new", None);  // (x, y), (h, w)
    env.insert(":1167", "Rect.from_vecs", None);  // pos, range.
    env.insert(":1168", "Rect.from_center", None);  // (x, y) size.
    env.insert(":1169", "Rect.move", None);  // rect, vec2.

    env.insert(":1172", "Vec2.add", None);
    env.insert(":1173", "Vec2.add_x_y", None);
    env.insert(":1174", "Vec2.add_x", None);
    env.insert(":1175", "Vec2.x", None);
    env.insert(":1176", "Vec2.x_list", None);
    env.insert(":1178", "Vec2.y", None);
    env.insert(":1179", "Vec2.add_y", None);
    env.insert(":1180", "Vec2.mul", None);
    env.insert(":1181", "Vec2.distance", None);

    env.insert(":1183", "Vec2List.map_add", None);
    env.insert(":1187", "Vec2List.map_add_x", None);
    env.insert(":1188", "Vec2List.map_add_y", None);

    // Construct the result to be passed to f38 func.
    env.insert(":1189", "Result.no_data", None);  // state -> Result.  Used for scene switching.
    env.insert(":1190", "Result.to_render", None);  // state, render_data -> Result.
    env.insert(":1191", "Result.to_send", None);  // state, send_data -> Result.

    // Image rendering library.
    env.insert(":1193", "Image.dot_line", None);  // p1, p2, freq. Dot per freq (and the end point).
    env.insert(":1194", "Image.line", None);
    env.insert(":1195", "Image.x_line", None);  // x, y, len
    env.insert(":1196", "Image.x_dot_line", None);
    env.insert(":1197", "Image.y_line", None);
    env.insert(":1198", "Image.center_rect_bound", None);  // size.
    env.insert(":1199", "Image.center_fill_rect", None);
    env.insert(":1200", "Image.rect_bound", None);  // x, y, w, h
    env.insert(":1201", "Image.fill_rect", None);  // x, y, w, h
    env.insert(":1202", "Image.fill_rect_aux", None);  // x, y, w, (h*w)
    env.insert(":1203", "Int.in_range", None);
    env.insert(":1204", "Image.contained_rect", Some(Type::Func(vec!((None, None), (None, None)), Box::new(Some(Type::Bool)))));  // p, rect (=((x, y), (w, h)))
    env.insert(":1205", "Image.make_bound", None);
    env.insert(":1206", "Image.from_uint", None);
    env.insert(":1207", "Bit.make", None);
    env.insert(":1208", "Bit.make_n", None);
    env.insert(":1209", "Bit.from_int", None);
    env.insert(":1210", "Int.ceil_sqr", None);
    env.insert(":1212", "Image.from_int_or_symbol", None);
    env.insert(":1213", "Image.from_int_or_symbol_with_size", None);
    env.insert(":1214", "Image.from_int", None);
    env.insert(":1215", "Image.from_int_left", None);
    env.insert(":1216", "Image.from_int_left_top", None);
    env.insert(":1217", "Image.from_int_with_size", None);
    env.insert(":1218", "Image.from_int_list", None);  // Draw in x-axis with 3 pixels mergin.
    env.insert(":1220", "Image.from_image_list", None);  // [Image] -> draw in x-axis with 3 pixels mergin.
    env.insert(":1221", "Image.from_image_list_with_mergin", None);  // [Image], x_offset, [x_offset]
    env.insert(":1222", "Image.from_bitmap_list", None);
    env.insert(":1224", "Image.bounding_box", None);
    env.insert(":1225", "Image.from_bitmap", None);
    env.insert(":1226", "Image.from_sparse_bitmap", None);

    // Optning scene.
    env.insert(":1227", "Opening.Scene", None);  // Opening.run, InitSceneState: [counter].
    env.insert(":1228", "Opening.run", None);
    env.insert(":1229", "Opening.draw_count_down", None);

    // History of games.
    env.insert(":1231", "History.InitSceneState", None);
    env.insert(":1232", "History.Scene", None);  // History.run, InitSceneState.
    env.insert(":1247", "History.HistoryList", None);  // In the reverse chronological order.
                                                        // [ID, game_id, top_team, bottom_team, which_team_continuted, ?]
    env.insert(":1253", "History.run", None);

    // Pelmanism game implementation.
    env.insert(":1305", "Pelmanism.Scene", None);  // Pelmanism.run, InitSceneState.
    env.insert(":1306", "Pelmanism.rotation_table", None);
    env.insert(":1303", "Pelmanism.size", None);
    env.insert(":1304", "Pelmanism.tiles", None);
    env.insert(":1307", "Pelmanism.KindTile", None);
    env.insert(":1308", "Pelmanism.KindGalaxy", None);

    env.insert(":1309", "Pelmanism.run", None);  // Takes orig: state, clicked: Vec2.
    env.insert(":1311", "Pelmanism.update_game", None);
    env.insert(":1312", "Pelmanism.solution_index", None);  // Returns the rotation index or -1 (fail).
    env.insert(":1313", "Pelmanism.is_solved", None); // bits1, bits2, rotation.
    env.insert(":1314", "Pelmanism.update", None);  // Takes orig: state, next: Pelmanism.state
    env.insert(":1315", "Pelmanism.draw", None);  // Takes Pelmanism.state
    env.insert(":1316", "Pelmanism.draw_tile", None);  // tile(int), index, offset, state.

    // Top level implementation.
    env.insert(":1328", "Garaxy.ModeOpening", None);
    env.insert(":1329", "Garaxy.ModeCariblation", None);
    env.insert(":1330", "Garaxy.ModeSpace", None);
    env.insert(":1331", "Garaxy.ModeTictactoe", None);
    env.insert(":1332", "Garaxy.ModePelmanism", None);
    env.insert(":1333", "Garaxy.ModeHistory", None);
    env.insert(":1334", "Garaxy.ModeTutorial", None);
    env.insert(":1335", "Garaxy.ModeError", None);

    env.insert(":1336", "Garaxy.Scenes", None);  // List of tasks
    env.insert(":1337", "Error.run", None);  // Print the error page.

    // Global entry point.
    env.insert(":1338", "Garaxy.run", None);
    env.insert(":1339", "Garaxy.next_scene", None);  // state, next_mode, next_scene_state -> Result.

    // state, clicked, 0, instances.
    env.insert(":1342", "Garaxy.run_internal", None);

    // Takes (state, clicked, Garaxy.scenes).
    // Dispatch scene.run based on the given state. If the result changes its scene,
    // re-dispatch to the new scene.
    env.insert(":1343", "Garaxy.dispatch", None);

    // Tutorial game page.
    env.insert(":1344", "Tutorial.Scene", None);
    env.insert(":1346", "Tutorial.run", None);

    // Main menu.
    env.insert(":1420", "Space.Scene", None);
    env.insert(":1427", "Space.run", None);

    // Calibration after the opening count down.
    env.insert(":1445", "Cariblation.Scene", None);
    env.insert(":1446", "Cariblation.run", None);

    // Tic-Tac-Toe game implementation.
    env.insert(":1451", "TicTacToe.Scene", None);

    env.insert(":1471", "TicTacToe.run", None);

    env.insert(":1472", "Interact.empty", None);
    env.insert(":1473", "Interact.seq2", None);
    env.insert(":1474", "Interact.seq3", None);
    env.insert(":1475", "Interact.seq4", None);
    env.insert(":1476", "Interact.seq", None);
    env.insert(":1477", "Interact.find_clicked", None);
    env.insert(":1478", "Interact.push_image", None);
    env.insert(":1490", "Interact.draw_clickable_at", None);

    let args = env::args().collect::<Vec<_>>();
    let content = fs::read_to_string(&args[1])?;
    for line in content.lines() {
        let parsed = parser::parse_line(line);
        eprintln!("Parsed: {:?}", parsed);
        let lambdified = lambda::lamdify(&parsed.value);
        eprintln!("Lambdified: {}", lambdified);
        let evaluated = lambda::eval(lambdified);
        eprintln!("Evaluated: {}", evaluated);
        let simplified = simplified::simplify(evaluated);
        eprintln!("Simplified: {}", simplified);
        let expr1 = expr::construct(&simplified, &env);
        eprintln!("Expr1 = {}", expr1);
        let expr2 = expr::simplify(&expr1);
        eprintln!("Expr2 = {}", expr2);
        let expr3 = expr::rename(&expr2, &env);
        let name = {
            if let Some(n) = env.get_name(&parsed.name) {
                n.clone()
            } else {
                parsed.name
            }
        };
        println!("{} = {}", name, expr3)
    }
    return Ok(());
}
