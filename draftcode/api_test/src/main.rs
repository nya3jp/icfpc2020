extern crate rust_game_base;

fn main() {
    rust_game_base::send_join_request();
    rust_game_base::send_start_request(&rust_game_base::Param {
        energy: 4,
        laser_power: 4,
        cool_down_per_turn: 4,
        life: 4,
    });
}
