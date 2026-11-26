#include <iostream>

#include <string>
#include <string_view>

std::string_view getYear(std::string_view fulldate) {
    return fulldate.substr(0, 4);
}

int main() {
    std::string date = "2025-11-25";
    std::string_view year = getYear(date);

    std::cout << year;

    return 0;
}