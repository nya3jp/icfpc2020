#include <iostream>
using namespace std;

bool isDraw(int b) {
  // row
  for (int i = 0; i < 3; ++i) {
    int r = (b >> (i * 3)) & 7;
    if (r == 0 || r == 7) return false;
  }

  // col
  for (int i = 0; i < 3; ++i) {
    int r = ((b>>i)&1) + ((b>>(i+3))&1) + ((b>>(i+6))&1);
    if (r == 0 || r == 3) return false;
  }

  // diag
  int r = (b&1) + ((b>>4)&1) + ((b>>8)&1);
  if (r == 0 || r == 3) return false;
  r = ((b>>2)&1) + ((b>>4)&1) + ((b>>6)&1);
  if (r == 0 || r == 3) return false;
  return true;
}

void printBoard(int b) {
  for (int i = 0; i < 3; ++i) {
    for (int j = 0; j < 3; ++j) {
      if ((b >> (i*3 + j))&1) cout << 'o';
      else cout << 'x';
    }
    cout << endl;
  }
}

int main() {
  for (int i = 0; i < (1<<9); ++i) {
    int cnt = 0;
    for (int j = 0; j < 10; ++j)
      if ((i>>j)&1) cnt++;
    if (cnt != 5) continue;
    if (isDraw(i)) {
      printBoard(i);
      cout << endl;
    }
  }
}
