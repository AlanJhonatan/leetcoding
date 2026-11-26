#include <iostream>
#include <unordered_map>
#include <string>

std::unordered_map<char, int> count(const std::string str) {
    std::unordered_map<char, int> freq;

    for(char c : str) {
        freq[c]++;
    }

    return freq;
}

void printMap(std::unordered_map<char, int> map) {
    for(const auto& [k, v] : map) {
        std::cout << k << ": " << v << "\n";
    }
}


int main() {
    std::string str = "hellow meus amores";
    std::unordered_map<char, int> freq = count(str);

    printMap(freq);
    
    return 0;
}