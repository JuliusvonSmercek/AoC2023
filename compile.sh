# in ~/.bash_aliases
# AoC 2023
compileCpp() {
  g++ -std=c++23 -g $1 -o program.out && ./program.out ${2-./sample.txt} && ./program.out ${3-./input.txt}
}

compileSpeedCpp() {
  g++ -std=c++23 -O3 -g $1 -o program.out && ./program.out ${2-./sample.txt} && ./program.out ${3-./input.txt}
}

compileDebugCpp() {
  g++ -Wall -Wno-unknown-pragmas -Wextra -Wconversion -Wshadow -fsanitize=undefined,address -D_GLIBCXX_DEBUG -std=c++23 -g $1 -o program.out && ./program.out ${2-./sample.txt} && ./program.out ${3-./input.txt}
}