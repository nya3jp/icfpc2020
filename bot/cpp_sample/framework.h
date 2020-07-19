#pragma once

#include "value.h"
#include "game.h"

constexpr int JOIN_REQUEST_TAG = 2;
constexpr int START_REQUEST_TAG = 3;

GameState parse_game_state(Value v) {
  GameState state;
  if (v.cdr_ && v.cdr_->car_ && v.cdr_->car_->kind_ == ValueKind::Number) {
    state.in_game = (v.cdr_->car_->number_ == 1);
  }
  return state;
}

Value send_and_receive(Value v) {
  const std::string &s = modulate(&v);
  std::cerr << "send: \"" << v.to_string() << "\"" << std::endl;
  std::cout << s << std::endl;
  std::string response;
  getline(std::cin, response);
  Value res = demodulate(response);
  std::cerr << "received: \"" << res.to_string() << "\"" << std::endl;
  return res;
}

Value send_join_request(int64_t player_key) {
  Value req = Value({JOIN_REQUEST_TAG, player_key, Value()});
  return send_and_receive(req);
}

GameState send_start_request(int64_t player_key, int equip1, int equip2, int equip3, int equip4) {
  Value equip = Value({equip1, equip2, equip3, equip4});
  Value req = Value({START_REQUEST_TAG, player_key, equip});
  Value res = send_and_receive(req);
  std::cerr << res.to_string() << std::endl;
  return parse_game_state(res);
}

GameState send_cmd(int64_t player_key, Command cmd) {
  // Do nothing.
  Value skip_turn = Value({4, player_key, Value()});
  Value res = send_and_receive(skip_turn);
  std::cerr << res.to_string() << std::endl;
  return parse_game_state(res);
}

void run_game(int64_t player_key, AIBase *ai) {
  StartParams params = ai->initialize();
  send_join_request(player_key);
  send_start_request(player_key, params.equip1, params.equip2, params.equip3, params.equip4);
  GameState state = {true};
  while (state.in_game) {
    Command cmd = ai->play(state);
    state = send_cmd(player_key, cmd);
  }
}
