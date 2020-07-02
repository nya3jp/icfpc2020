#include <iostream>
#include <string>

#include <cpr/cpr.h>

int main(int argc, char* argv[]) {
    const std::string server_url{argv[1]};
    const std::string player_key{argv[2]};

    std::cout << "ServerUrl: " << server_url << "; PlayerKey: " << player_key << std::endl;

    auto res = cpr::Get(cpr::Url{argv[1]}, cpr::Parameters{{"playerKey", argv[2]}});
    std::cout << res.status_line << std::endl;
    return 0;
}
