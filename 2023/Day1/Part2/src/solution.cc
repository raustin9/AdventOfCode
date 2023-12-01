#include <cctype>
#include <cstdint>
#include <iostream>
#include <stdexcept>
#include <vector>
#include <fstream>

int 
main(void) {
    std::vector<std::pair<int32_t, int32_t>> values;
    int32_t first, last;
    uint32_t total = 0;
    std::string input;

    while (std::getline(std::cin, input)) {
        first = -1;
        for (size_t j = 0; j < input.length(); j++) {
            if (isdigit(input[j])) {
                last = input[j] - '0';
                if (first == -1) {
                    first = input[j] - '0'; 
                }
            }
        }
        values.push_back(std::make_pair(first, last));
    }


    for (size_t i = 0; i < values.size(); i++) {
        total += (10 * values[i].first) + (values[i].second);
    }

    std::cout << "Total: " << total << std::endl;

    return 0;
}
