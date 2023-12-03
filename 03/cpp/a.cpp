#include "util.h"

bool isSymbol(const char c) { return !isdigit(c) && '.' != c; }

long long solve(vector<string> &lines) {
  long long result = 0;

  for (int i = 0; i < lines.size(); ++i) {
    int j = 0;
    while (j < lines[i].size()) {
      while (j < lines[i].size() && !isdigit(lines[i][j])) {
        ++j;
      }
      int number = 0;
      int startJ = j - 1;
      while (j < lines[i].size() && isdigit(lines[i][j])) {
        number = number * 10 + (lines[i][j] - '0');
        ++j;
      }
      int endJ = j - 1;
      bool condition = false;
      for (int i1 = max(i - 1, 0); i1 <= min(i + 1, (int)lines.size() - 1); ++i1) {
        for (int j1 = max(startJ, 0); j1 <= min(j, (int)lines[i1].size() - 1); ++j1) {
          condition |= isSymbol(lines[i1][j1]);
        }
      }
      if (condition) {
        result += number;
      }
    }
  }

  return result;
}