import {evaluate} from './eval';
import {Picture} from './picture';

export type Value = NumberValue | FuncValue;

export interface NumberValue {
    kind: 'number'
    number: bigint
}

export interface FuncValue {
    kind: 'func'
    func: (env: Environment, expr: Expr) => Expr
}

export interface Point {
    x: number
    y: number
}

export function isNil(env: Environment, expr: Expr): boolean {
    const nilValue = BigInt(123);
    const nonNilValue = BigInt(456);
    const result = evaluate(env, makeApply(makeApply(makeApply(expr, makeReference('_isnil_helper')), makeNumber(nilValue)), makeNumber(nonNilValue)));
    if (result.kind !== 'number') {
        throw new Error('Not nil/cons');
    }
    switch (result.number) {
        case nilValue:
            return true
        case nonNilValue:
            return false;
        default:
            throw new Error('Not nil/cons');
    }
}

type Thunk = ApplyThunk | ReferenceThunk;

export interface ApplyThunk {
    kind: 'apply'
    lhs?: Expr
    rhs?: Expr
    cache?: Value
}

export interface ReferenceThunk {
    kind: 'reference'
    name: string
    cache?: Value
}

export function debugString(env: Environment, expr: Expr): string {
    switch (expr.kind) {
        case 'number':
            return String(expr.number);
        case 'func':
            if (isNil(env, expr)) {
                return 'nil'
            }
            const car = evaluate(env, makeApply(makeReference('car'), expr));
            const cdr = evaluate(env, makeApply(makeReference('cdr'), expr));
            return `ap ap cons ${debugString(env, car)} ${debugString(env, cdr)}`;
        default:
            return debugString(env, evaluate(env, expr));
    }
}

// human readable version of debugString.
export function debugListString(env: Environment, expr: Expr): string {
    switch (expr.kind) {
        case 'number':
            return String(expr.number);
        case 'func':
            if (isNil(env, expr)) {
                return 'nil'
            }
            const car = evaluate(env, makeApply(makeReference('car'), expr));
            const cdr = evaluate(env, makeApply(makeReference('cdr'), expr));
            return `( ${debugListString(env, car)}, ${debugListString(env, cdr)} )`;
        default:
            return debugListString(env, evaluate(env, expr));
    }
}

export type Expr = Value | Thunk;

export function makeApply(lhs: Expr, rhs: Expr): ApplyThunk {
    return {kind: 'apply', lhs, rhs};
}

export function makeReference(name: string): ReferenceThunk {
    return {kind: 'reference', name};
}

export function makeNumber(i: bigint): NumberValue {
    return {kind: 'number', number: i};
}

export function makeBoolean(b: boolean): ReferenceThunk {
    return makeReference(b ? 't' : 'f');
}

export function makePoint(p: Point): Expr {
    return makeApply(makeApply(makeReference('cons'), makeNumber(BigInt(p.x))), makeNumber(BigInt(p.y)));
}

export function makeList(exprs: Array<Expr>): Expr {
    if (exprs.length === 0) {
        return makeReference('nil')
    }
    return makeApply(makeApply(makeReference('cons'), exprs[0]), makeList(exprs.slice(1)));
}

export function parseList(env: Environment, value: Value): Array<Value> {
    const elems = [];
    for (let cur = value; !isNil(env, cur); cur = evaluate(env, makeApply(makeReference('cdr'), cur))) {
        const car = evaluate(env, makeApply(makeReference('car'), cur));
        elems.push(car);
    }
    return elems;
}

export type Environment = Map<string, Expr>;

// Pretty representation of modulatable data.
export type PrettyData = NumberData | ListData | ConsData;

export interface NumberData {
    kind: 'number'
    number: bigint
}

export interface ListData {
    kind: 'list'
    elems: Array<PrettyData>
}

export interface ConsData {
    kind: 'cons'
    car: PrettyData
    cdr: PrettyData
}

export function valueToPrettyData(env: Environment, value: Value): PrettyData {
    switch (value.kind) {
        case 'number':
            return {kind: 'number', number: value.number}
        case 'func':
            const elems: Array<PrettyData> = [];
            let cur: Value = value;
            while (true) {
                if (cur.kind !== 'func') {
                    elems.push(valueToPrettyData(env, cur));
                    return elems.reduceRight((cdr, car) => ({kind: 'cons', car, cdr}));
                }
                if (isNil(env, cur)) {
                    return {kind: 'list', elems};
                }
                const car = evaluate(env, makeApply(makeReference('car'), cur));
                elems.push(valueToPrettyData(env, car));
                const cdr = evaluate(env, makeApply(makeReference('cdr'), cur));
                cur = cdr;
            }
    }
}

export function prettyDataEqual(a: PrettyData, b: PrettyData): boolean {
    switch (a.kind) {
        case 'number':
            if (b.kind !== 'number') {
                return false;
            }
            return a.number === b.number;
        case 'list':
            if (b.kind !== 'list') {
                return false;
            }
            if (a.elems.length !== b.elems.length) {
                return false;
            }
            for (let i = 0; i < a.elems.length; ++i) {
                if (!prettyDataEqual(a.elems[i], b.elems[i])) {
                    return false;
                }
            }
            return true;
        case 'cons':
            if (b.kind !== 'cons') {
                return false;
            }
            return prettyDataEqual(a.car, b.car) && prettyDataEqual(a.cdr, b.cdr);
    }
}

export function prettyDataString(data: PrettyData): string {
    switch (data.kind) {
        case 'number':
            return String(data.number);
        case 'list':
            return `[${data.elems.map(prettyDataString).join(', ')}]`;
        case 'cons':
            return `(${prettyDataString(data.car)} . ${prettyDataString(data.cdr)})`;
    }
}
