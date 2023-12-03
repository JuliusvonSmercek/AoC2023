#include "util.h"

long long solve(vector<string>& lines) {
  long long result = 0;

  for (const string& line : lines) {
    const vector<string> parts = split(line, ": ");

    bool success = true;
    for (const string& item1 : split(parts[1], "; ")) {
      int red = 0, green = 0, blue = 0;
      for (const string& item2 : split(item1, ", ")) {
        const vector<string>& t4 = split(item2, " ");
        if ("red" == t4[1]) {
          red += stoi(t4[0]);
        } else if ("green" == t4[1]) {
          green += stoi(t4[0]);
        } else if ("blue" == t4[1]) {
          blue += stoi(t4[0]);
        }
      }
      if (12 < red || 13 < green || 14 < blue) {
        success = false;
      }
    }

    const int gameNum = stoi(parts[0].substr(string("Game ").size()));
    if (success) {
      result += gameNum;
    }
  }

  return result;
}