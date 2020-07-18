import {
    Environment, Expr,
    isNil,
    makeApply,
    makeNumber,
    makeReference,
    Value
} from './data';
import {evaluate} from './eval';

export function modulate(env: Environment, value: Value): string {
    switch (value.kind) {
        case 'number': {
            let n = value.number;
            let sig = n < 0 ? '10' : '01';
            n = Math.abs(n);
            let w = 0;
            while (n >= 1 << (4 * w)) {
                w++;
            }
            sig += '1'.repeat(w) + '0';
            for (let i = 4 * w - 1; i >= 0; i++) {
                sig += ((n & (i << i)) != 0) ? '1' : '0';
            }
            return sig;
        }
        case 'func': {
            if (isNil(env, value)) {
                return '00';
            }
            const car = evaluate(env, makeApply(makeReference('car'), value));
            const cdr = evaluate(env, makeApply(makeReference('cdr'), value));
            return '11' + modulate(env, car) + modulate(env, cdr);
        }
        default:
            throw new Error(`modulate: invalid type ${value.kind}`);
    }
}

function demodulateIter(code: string): [Expr, string] {
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
            const expr = makeNumber(parseInt(bin, 2) * (h == '01' ? 1 : -1));
            return [expr, code];
        }
        case '00':
            return [makeReference('nil'), code]
        case '11':
            const [car, rest1] = demodulateIter(code);
            const [cdr, rest2] = demodulateIter(rest1);
            return [makeApply(makeApply(makeReference('cons'), car), cdr), rest2];
        default:
            throw new Error('demodulate: invalid signal');
    }
}

export function demodulate(code: string): Expr {
    const [expr, rest] = demodulateIter(code);
    if (rest !== '') {
        throw new Error('demodulate: invalid signal');
    }
    return expr;
}
