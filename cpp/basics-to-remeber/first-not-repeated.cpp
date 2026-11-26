#include <iostream>
#include <string>
#include <unordered_map>

int firstUniq(const std::string str) {
    std::unordered_map<char, int> map;

    for(char c : str) map[c]++;

    for(int i = 0; i < str.size(); i++) {
        char c = str[i];

        if(map[c] == 1) return i;
    }

    return -1;
}

int main() {
    std::string str = "loveleetcode";
    int idx = firstUniq(str);

    std::cout << "first uniq: " << idx << "\n";

    return 0;
}