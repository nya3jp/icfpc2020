#pragma once

#include "value.h"

struct StartParams {
    int equip1;
    int equip2;
    int equip3;
    int equip4;
};

struct GameState {
  bool in_game;
};

struct Command {
};

class AIBase {
public:
    virtual ~AIBase() = default;
    virtual StartParams initialize() = 0;
    virtual Command play(GameState state) = 0;
};
