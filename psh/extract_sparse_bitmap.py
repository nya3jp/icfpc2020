# Copyright 2020 Google LLC
# Copyright 2020 Team Spacecat
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import itertools
import sys


def _find_line(lines, tag):
    for line in lines:
        if line.startswith(tag + ' = '):
            return eval(line[len(tag + ' = '):])


def grouper(iterable, n, fillvalue=None):
    args = [iter(iterable)] * n
    return itertools.zip_longest(*args, fillvalue=fillvalue)


def _draw(lines, tag, output):
    data = _find_line(lines, tag)

    bitmap = [['0'] * 4096 for _ in range(4096)]
    for pos in data:
        x = pos // 4096
        y = pos % 4096
        bitmap[y][x] = '1'

    xmin, xmax, ymin, ymax = 4096, 0, 4096, 0
    for y, row in enumerate(bitmap):
        found = False
        for x, c in enumerate(row):
            if c != '0':
                xmin = min(xmin, x)
                xmax = max(xmax, x)
                found = True
        if found:
            ymin = min(ymin, y)
            ymax = max(ymax, y)

    with open(output, 'w') as f:
        f.write('P1\n%d %d\n' % (xmax - xmin + 1, ymax - ymin + 1))
        for row in bitmap[ymin:ymax+1]:
            f.write('%s\n' % ' '.join(row[xmin:xmax+1]))


def main():
    with open(sys.argv[1], 'r') as f:
        lines = f.readlines()
    for arg in sys.argv[2:]:
        _draw(lines, arg, arg+'.pbm')


if __name__ == '__main__':
    main()
