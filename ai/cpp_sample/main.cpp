#include <iostream>

#include "value.h"
#include "framework.h"


int main(int argc, char **argv) {
  int64_t player_key = std::stoll(argv[1]);

  send_join_request(player_key);
  send_start_request(player_key, 4, 4, 4, 4);

  while (true) {
    GameState state = send_cmd(player_key);
    if (!state.in_game) break;
  }
  return 0;
}
