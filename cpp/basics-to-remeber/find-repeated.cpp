#include <iostream>
#include <string>
#include <unordered_map>

void findRepeated(const std::string str) {
    std::unordered_map<char, int> char_freq;

    for(char c : str) char_freq[c]++;

    for(const auto& [k, v] : char_freq) {
        if(v > 1) {
            std::cout << k << ": " << v << "\n"; 
        }
    }
}

int main(int argc, char const *argv[]) {
    std::string str = "loveleetcode";

    findRepeated(str);
    return 0;
}
