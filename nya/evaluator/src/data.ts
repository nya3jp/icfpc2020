import {evaluate} from './eval';

export type Value = NumberValue | FuncValue | PictureValue;

export interface NumberValue {
    kind: 'number'
    number: number
}

export interface FuncValue {
    kind: 'func'
    func: (env: Environment, expr: Expr) => Expr
}

export interface PictureValue {
    kind: 'picture'
    points: Array<Point>
}

export interface Point {
    x: number
    y: number
}

export function isNil(env: Environment, expr: Expr): boolean {
    const result = evaluate(env, makeApply(makeApply(makeApply(expr, makeReference('_isnil_helper')), makeNumber(123)), makeNumber(456)));
    if (result.kind !== 'number') {
        throw new Error('Not nil/cons');
    }
    switch (result.number) {
        case 123:
            return true
        case 456:
            return false;
        default:
            throw new Error('Not nil/cons');
    }
}

type Thunk = ApplyThunk | ReferenceThunk;

export interface ApplyThunk {
    kind: 'apply'
    lhs: Expr
    rhs: Expr
}

export interface ReferenceThunk {
    kind: 'reference'
    name: string
}

export function debugString(env: Environment, expr: Expr): string {
    switch (expr.kind) {
        case 'number':
            return String(expr.number);
        case 'func':
            const elems = [];
            for (let cur: Value = expr; !isNil(env, cur); cur = evaluate(env, makeApply(makeReference('cdr'), cur))) {
                const car = evaluate(env, makeApply(makeReference('car'), cur));
                elems.push(debugString(env, car));
            }
            return `[${elems.join(' ')}]`;
        case 'picture':
            return '<picture>';
        case 'apply':
            return `(${debugString(env, expr.lhs)} ${debugString(env, expr.rhs)})`;
        case 'reference':
            return expr.name;
    }
}

export type Expr = Value | Thunk;

export function makeApply(lhs: Expr, rhs: Expr): ApplyThunk {
    return {kind: 'apply', lhs, rhs};
}

export function makeReference(name: string): ReferenceThunk {
    return {kind: 'reference', name};
}

export function makeNumber(i: number): NumberValue {
    return {kind: 'number', number: i};
}

export function makeBoolean(b: boolean): ReferenceThunk {
    return makeReference(b ? 't' : 'f');
}

export function makePicture(points: Array<Point>): PictureValue {
    return {kind: 'picture', points};
}

export function makeList(exprs: Array<Expr>): Expr {
    if (exprs.length === 0) {
        return makeReference('nil')
    }
    return makeApply(makeApply(makeReference('cons'), exprs[0]), makeList(exprs.slice(1)));
}

export type Environment = Map<string, Expr>;
