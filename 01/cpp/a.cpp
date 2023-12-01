#include "util.h"

int processStr(const string &line, const bool smaller) {
  size_t index = 0;
  int result = -1;
  const vector<pair<int, string>> numbers{{0, "0"}, {1, "1"}, {2, "2"}, {3, "3"}, {4, "4"},
                                          {5, "5"}, {6, "6"}, {7, "7"}, {8, "8"}, {9, "9"}};
  for (const auto &[value, repr] : numbers) {
    const size_t pos = (smaller) ? line.find(repr) : line.rfind(repr);
    if (pos == line.npos) {
      continue;
    }
    if (-1 == result || ((smaller) ? pos < index : pos > index)) {
      index = pos;
      result = value;
    }
  }
  assert(-1 != result);
  return result;
}

long long solve(vector<string> &lines) {
  long long result = 0;

  for (const string &line : lines) {
    result += processStr(line, true) * 10 + processStr(line, false);
  }

  return result;
}