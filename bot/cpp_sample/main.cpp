#include <iostream>

#include "value.h"
#include "framework.h"
#include "game.h"


class AI : public AIBase {
public:
    ~AI() = default;
    StartParams initialize() {
    }
    Command play(GameState state) {
    }
};


int main(int argc, char **argv) {
  int64_t player_key = std::stoll(argv[1]);
  AI ai;
  run_game(player_key, &ai);
  return 0;
}
