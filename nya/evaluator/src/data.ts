import {evaluate} from './eval';

export type Value = NumberValue | FuncValue | PictureValue;

interface NumberValue {
    kind: 'number'
    number: number
}

interface FuncValue {
    kind: 'func'
    func: (env: Environment, expr: Expr) => Expr
}

interface PictureValue {
    kind: 'picture'
}

export function isNil(env: Environment, value: Value): boolean {
    const result = evaluate(env, makeApply(makeApply(makeApply(makeLiteral(value), makeReference('_isnil_helper')), makeNumber(123)), makeNumber(456)));
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

export function valueToString(env: Environment, value: Value): string {
    switch (value.kind) {
        case 'number':
            return String(value.number);
        case 'func':
            const elems = [];
            for (let cur: Value = value; !isNil(env, cur); cur = evaluate(env, makeApply(makeReference('cdr'), makeLiteral(cur)))) {
                const car = evaluate(env, makeApply(makeReference('car'), makeLiteral(cur)));
                elems.push(valueToString(env, car));
            }
            return `[${elems.join(' ')}]`;
        case 'picture':
            return '<picture>';
    }
}

export type Expr = ApplyExpr | ReferenceExpr | LiteralExpr;

interface ApplyExpr {
    kind: 'apply'
    lhs: Expr
    rhs: Expr
}

interface ReferenceExpr {
    kind: 'reference'
    name: string
}

interface LiteralExpr {
    kind: 'literal'
    value: Value
}

export function exprToString(env: Environment, expr: Expr): string {
    switch (expr.kind) {
        case 'apply':
            return `(${exprToString(env, expr.lhs)} ${exprToString(env, expr.rhs)})`;
        case 'reference':
            return expr.name;
        case 'literal':
            return valueToString(env, expr.value);
    }
}

export function makeApply(lhs: Expr, rhs: Expr): Expr {
    return {kind: 'apply', lhs, rhs};
}

export function makeReference(name: string): Expr {
    return {kind: 'reference', name};
}

export function makeLiteral(value: Value): Expr {
    return {kind: 'literal', value: value};
}

export function makeNumber(i: number): Expr {
    return makeLiteral({kind: 'number', number: i});
}

export function makeBoolean(b: boolean): Expr {
    return makeReference(b ? 't' : 'f');
}

export function makePicture(): Expr {
    return makeLiteral({kind: 'picture'});
}

export function makeList(exprs: Array<Expr>): Expr {
    if (exprs.length === 0) {
        return makeReference('nil')
    }
    return makeApply(makeApply(makeReference('cons'), exprs[0]), makeList(exprs.slice(1)));
}

export type Environment = Map<string, Expr>;
