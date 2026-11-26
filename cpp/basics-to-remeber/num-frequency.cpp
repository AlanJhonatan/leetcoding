#include <iostream>
#include <vector>
#include <unordered_map>

std::unordered_map<int, int> charsMap(const std::vector<int> v) {
    std::unordered_map<int, int> freq;

    for(int n : v) {
        freq[n]++;
    }

    return freq;
}

void printHashmap (const std::unordered_map<int, int> hashmap) {
    for (const auto& [k, v]: hashmap) {
        std::cout << "[" << k << "]: " << v << "\n";
    }
}

int main () {
    std::vector<int> v = { 2, 2, 3 };
    std::unordered_map hashmap = charsMap(v);

    printHashmap(hashmap);

    return 0;
}