#include "util.h"

bool isSymbol(const char c) { return !isdigit(c) && '.' != c; }

long long solve(vector<string> &lines) {
  long long result = 0;

  const int n = lines.size();
  const int m = lines[0].size();

  vector<vector<vector<long long>>> matrix(n, vector<vector<long long>>(m));
  for (int i = 0; i < n; ++i) {
    int j = 0;
    while (j < m) {
      while (j < m && !isdigit(lines[i][j])) {
        ++j;
      }
      int number = 0;
      int startJ = j;
      while (j < m && isdigit(lines[i][j])) {
        number = number * 10 + (lines[i][j] - '0');
        ++j;
      }
      int endJ = j - 1;
      for (int i1 = i - 1; i1 <= i + 1; ++i1) {
        for (int j1 = startJ - 1; j1 <= endJ + 1; ++j1) {
          if (0 <= i1 && i1 < n && 0 <= j1 && j1 < m && '*' == lines[i1][j1]) {
            matrix[i1][j1].push_back({number});
          }
        }
      }
    }
  }
  for (int j = 0; j < n; ++j) {
    for (int i = 0; i < m; ++i) {
      if (2 == matrix[j][i].size()) {
        result += matrix[j][i][0] * matrix[j][i][1];
      }
    }
  }

  return result;
}