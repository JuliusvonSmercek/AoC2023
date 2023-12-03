#include "util.h"

long long solve(vector<string>& lines) {
  long long result = 0;

  for (const string& line : lines) {
    const vector<string> parts = split(line, ": ");

    int maxRed = 0, maxGreen = 0, maxBlue = 0;
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
      maxRed = max(maxRed, red);
      maxBlue = max(maxBlue, blue);
      maxGreen = max(maxGreen, green);
    }

    result += maxRed * maxBlue * maxGreen;
  }

  return result;
}