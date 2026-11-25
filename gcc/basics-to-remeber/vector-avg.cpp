// #include <bits/stdc++.h>
#include <iostream>
#include <vector>

double calculateVectorAvg(const std::vector<int>& numbers) {
    if(numbers.empty()) return 0.0;

    long long soma = 0;

    for (int num : numbers) {
        soma += num;
    }

    return static_cast<double>(soma) / numbers.size();
}


int main() {
    std::vector<int> input = { 1, 2, 3, 4, 5};
    std::cout << "average: " << calculateVectorAvg(input) << "\n";

    return 0;
}
