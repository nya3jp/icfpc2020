#include <iostream>
#include <string>

int main(int argc, char* argv[]) {
  const std::string player_key{argv[1]};

  std::cout << player_key << std::endl;
  std::string response;
  std::getline(std::cin, response);

  std::cerr << "Server response: " << response << std::endl;
  return 0;
}
