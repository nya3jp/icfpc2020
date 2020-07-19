#include <iostream>

#include "value.h"

constexpr int JOIN_REQUEST_TAG = 2;
constexpr int START_REQUEST_TAG = 3;

struct GameState {
  bool in_game;
};

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
  Value req = Value(Value(JOIN_REQUEST_TAG),
                    Value(Value(player_key),
                          Value(Value(), Value())));
  return send_and_receive(req);
}

GameState send_start_request(int64_t player_key, int equip1, int equip2, int equip3, int equip4) {
  Value equip = Value(Value(equip1),
                      Value(Value(equip2),
                            Value(Value(equip3),
                                  Value(Value(equip4),
                                        Value()))));
  Value req = Value(Value(START_REQUEST_TAG),
                    Value(Value(player_key),
                          Value(equip,
                                Value())));
  Value res = send_and_receive(req);
  std::cerr << res.to_string() << std::endl;
  return parse_game_state(res);
}

GameState send_cmd(int64_t player_key) {
  // Do nothing.
  Value skip_turn = Value(Value(4),
                          Value(Value(player_key),
                                Value(Value(),
                                      Value())));
  Value res = send_and_receive(skip_turn);
  std::cerr << res.to_string() << std::endl;
  return parse_game_state(res);
}

int main(int argc, char **argv) {
  int64_t player_key = std::stoll(argv[1]);

  send_join_request(player_key);
  send_start_request(player_key, 4, 4, 4, 4);

  while (true) {
    GameState state = send_cmd(player_key);
    if (!state.in_game) break;
  }

  // run_modulate_test();
  // run_demodulate_test();
  return 0;
}
