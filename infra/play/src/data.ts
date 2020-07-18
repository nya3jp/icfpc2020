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

type Thunk = ApplyThunk | ReferenceThunk | SideEffectThunk;

export interface ApplyThunk {
    kind: 'apply'
    lhs: Expr
    rhs: Expr
    cache?: Value
}

export interface ReferenceThunk {
    kind: 'reference'
    name: string
    cache?: Value
}

export interface SideEffectThunk {
    kind: 'sideEffect'
    expr: Expr
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
        case 'picture':
            return '<picture>';
        case 'apply':
            return `(${debugString(env, expr.lhs)} ${debugString(env, expr.rhs)})`;
        case 'reference':
            return expr.name;
        case 'sideEffect':
            return debugString(env, expr.expr);
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

export function makeSideEffect(e: Expr): SideEffectThunk {
    return {kind: 'sideEffect', expr: e};
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
