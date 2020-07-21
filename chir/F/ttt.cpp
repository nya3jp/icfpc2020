// Copyright 2020 Google LLC
// Copyright 2020 Team Spacecat
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
