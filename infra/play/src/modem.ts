/**
 * Copyright 2020 Google LLC
 * Copyright 2020 Team Spacecat
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import {
    evaluate,
    Expr,
    isNil,
    makeCar,
    makeCdr,
    makeCons,
    makeNil,
    makeNumber,
    Value,
} from './data';

export function modulate(expr: Expr): string {
    const value = evaluate(expr);
    switch (value.kind) {
        case 'number': {
            let n = value.number;
            let sig = n < 0 ? '10' : '01';
            if (n < 0) {
                n = -n;
            }
            let w = 0;
            while (n >= BigInt(1) << BigInt(4 * w)) {
                w++;
            }
            sig += '1'.repeat(w) + '0';
            for (let i = 4 * w - 1; i >= 0; i--) {
                sig += ((n & (BigInt(1) << BigInt(i))) > 0) ? '1' : '0';
            }
            return sig;
        }
        case 'nil':
        case 'cons':
        case 'func': {
            if (isNil(value)) {
                return '00';
            }
            return '11' + modulate(makeCar(value)) + modulate(makeCdr(value));
        }
    }
}

function demodulateIter(code: string): [Value, string] {
    const h = code.slice(0, 2);
    code = code.slice(2);
    switch (h) {
        case '01':
        case '10': {
            let w = 0;
            while (code.charAt(w) === '1') {
                w++;
            }
            code = code.slice(w+1);
            const bin = code.slice(0, 4*w);
            code = code.slice(4*w);
            const value = makeNumber(w > 0 ? BigInt('0b' + bin) * BigInt(h === '01' ? 1 : -1) : BigInt(0));
            return [value, code];
        }
        case '00':
            return [makeNil(), code]
        case '11':
            const [car, rest1] = demodulateIter(code);
            const [cdr, rest2] = demodulateIter(rest1);
            return [makeCons(car, cdr), rest2];
        default:
            throw new Error('demodulate: invalid signal');
    }
}

export function demodulate(code: string): Value {
    const [expr, rest] = demodulateIter(code);
    if (rest !== '') {
        throw new Error('demodulate: invalid signal');
    }
    return expr;
}
