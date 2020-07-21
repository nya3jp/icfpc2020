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
