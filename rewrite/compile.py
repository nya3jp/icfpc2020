#!/usr/bin/env python3
#
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

import abc
import functools
import itertools
import sys
import typing
from typing import List, Tuple

import PIL.Image


class Expr(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def __str__(self) -> str:
        ...

    @staticmethod
    def parse(code: str) -> 'Expr':
        code = code.replace('(', ' ( ').replace(')', ' ) ')
        expr, rest = Expr._parse_iter(code)
        assert not rest, rest
        return expr

    @staticmethod
    def _parse_iter(code: str) -> Tuple['Expr', str]:
        elems = []
        while True:
            code = code.lstrip()
            if not code or code.startswith(')'):
                break
            elif code.startswith('('):
                elem, code = Expr._parse_iter(code[1:])
                elems.append(elem)
                code = code.lstrip()
                assert code.startswith(')'), code
                code = code[1:]
            else:
                split = code.split(None, 1)
                if len(split) == 1:
                    split.append('')
                elem, code = split
                elems.append(Atom(elem))
        return functools.reduce(Ap, elems), code

    def lift_arg(self, arg: 'Atom') -> 'Expr':
        expr, want = self._lift_arg(arg)
        if want:
            return expr
        return Ap('t', expr)

    @abc.abstractmethod
    def _lift_arg(self, arg: 'Atom') -> ('Expr', bool):
        ...


class Ap(Expr):
    lhs: Expr
    rhs: Expr

    def __init__(self, lhs: Expr, rhs: Expr):
        self.lhs = lhs
        self.rhs = rhs

    def __str__(self) -> str:
        return 'ap %s %s' % (self.lhs, self.rhs)

    def _lift_arg(self, arg: 'Atom') -> (Expr, bool):
        lhs, lhs_want = self.lhs._lift_arg(arg)
        rhs, rhs_want = self.rhs._lift_arg(arg)
        if lhs_want and rhs_want:
            return Ap(Ap(Atom('s'), lhs), rhs), True
        if lhs_want:
            return Ap(Ap(Atom('c'), lhs), rhs), True
        if rhs_want:
            return Ap(Ap(Atom('b'), lhs), rhs), True
        return Ap(lhs, rhs), False


class Atom(Expr):
    name: str

    def __init__(self, name: str):
        self.name = name

    def __str__(self) -> str:
        return self.name

    def _lift_arg(self, arg: 'Atom') -> (Expr, bool):
        if arg.name == self.name:
            return Atom('i'), True
        return self, False


CODE = """
and a b = a b f
cdar a = car (cdr a)
cdaar a = car (car (cdr a))
cddar a = car (cdr (cdr a))
cdddr a = cdr (cdr (cdr a))
map f x = isnil x nil (cons (f (car x)) (map f (cdr x)))
mod a b = add a (neg (mul (div a b) b))
cdadaar a = car (car (cdr (car (cdr a))))
mayberewrite res = (and (and (eq (car res) 0) (eq (cdaar res) 2)) (eq (cdadaar res) 1)) (rewrite res) res
rewrite res = cons (car res) (cons (cdar res) (cons (rewritepics (cddar res)) (cdddr res)))
rewritepics pics = cons (mergepics (decode pic0) (car pics)) (cons (mergepics (decode pic1) (cdar pics)) (cons (mergepics (decode pic2) (cddar pics)) (cdddr pics)))
mergepics a b = isnil a b (cons (car a) (mergepics (cdr a) b))
translate p = cons (neg (car p)) (cdr p)
decode in = map translate (decode' nil in)
decode' out in = isnil in out (decode' (decodeone out (car in)) (cdr in))
decodeone out value = decodeone' out (mod (div value 100) 1000) (div value 100000) (mod value 100)
decodeone' out x y l = (eq l 0) out (decodeone' (cons (cons x y) out) (inc x) y (dec l))
galaxy state input = mayberewrite (:1338 state input)
"""


def main():
    # Convert image...
    im = PIL.Image.open('spacecat-3bit.png')
    im = im.convert('RGB')
    w, h = im.size
    for ch in range(3):
        lim = im.getchannel(ch)
        data = []
        for y in range(h):
            x = 0
            while x < w:
                if lim.getpixel((x, y)) == 0:
                    x += 1
                    continue
                l = 1
                while l < 99 and x + l < w and lim.getpixel((x+l, y)):
                    l += 1
                data.append((y * 1000 + x) * 100 + l)
                x += l
        print('pic%d = %snil' % (ch, ''.join('ap ap cons %d ' % x for x in data)))

    # Compile CODE.
    for line in CODE.strip().splitlines():
        line = line.strip()
        if not line or line.startswith('#'):
            continue

        lhs_str, rhs_str = line.split('=')

        lhs_str = lhs_str.strip()
        lhs_name = lhs_str.split()[0]
        args = [Atom(name) for name in lhs_str.split()[1:]]

        rhs_str = rhs_str.strip()
        rhs = Expr.parse(rhs_str)

        for arg in reversed(args):
            rhs = rhs.lift_arg(arg)
        print('%s = %s' % (lhs_name, rhs))


if __name__ == '__main__':
    main()
