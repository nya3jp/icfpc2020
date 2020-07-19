extern crate rust_game_base;

fn main() {
    rust_game_base::send_join_request();
    rust_game_base::send_start_request(4, 4, 4, 4);
}
