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
    size = data[0]
    bits = ''.join(''.join(reversed('{:063b}'.format(v))) for v in data[1:])
    rows = list(grouper(bits, size, '0'))
    rows = list(reversed(list(
        itertools.dropwhile(lambda row: all(c == '0' for c in row),
                            reversed(rows)))))
    with open(output, 'w') as f:
        f.write('P1\n%d %d\n' % (size, len(rows)))
        for row in rows:
            f.write('%s\n' % ' '.join(row))


def main():
    with open(sys.argv[1], 'r') as f:
        lines = f.readlines()
    for arg in sys.argv[2:]:
        _draw(lines, arg, arg+'.pbm')


if __name__ == '__main__':
    main()
