#include <cpr/cpr.h>

#include <iostream>
#include <string>

int main(int argc, char* argv[]) {
  const std::string server_url{argv[1]};
  const std::string player_key{argv[2]};

  std::cout << "ServerUrl: " << server_url << "; PlayerKey: " << player_key
            << std::endl;

  auto res =
      cpr::Post(cpr::Url{server_url}, cpr::Body{"(2, " + player_key + ", ())"},
                cpr::Header{{"Content-Type", "text/plain"}});
  if (res.status_code != 200) {
    std::cout << "Unexpected server response:\nHTTP code: " << res.status_code
              << "\nResponse body: " << res.text << std::endl;
    return 2;
  }

  std::cout << "Server response: " << res.text << std::endl;
  return 0;
}
