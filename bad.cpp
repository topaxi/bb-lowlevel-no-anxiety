// clang++ -Wall -Wextra -O2 bad.cpp -o bad
#include <iostream>
#include <vector>

int main() {
  std::vector<int> xs = {1, 2, 3};
  int &x = xs[0];
  xs.push_back(4);
  std::cout << x << "\n";
}
