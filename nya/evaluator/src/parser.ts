import {newStandardEnvironment} from './builtins';
import {Environment, Expr} from './data';

function parseExprIter(tokens: Array<string>): [Expr, Array<string>] {
    const token = tokens[0];
    if (token === 'ap') {
        let [lhs, rest1] = parseExprIter(tokens.slice(1));
        let [rhs, rest2] = parseExprIter(rest1);
        return [{kind: 'apply', lhs, rhs}, rest2];
    }
    if (/^-?\d+$/.test(token)) {
        return [{kind: 'literal', value: {kind: 'number', number: parseInt(token)}}, tokens.slice(1)];
    }
    return [{kind: 'reference', name: token}, tokens.slice(1)];
}

export function parseExpr(code: string): Expr {
    const tokens = code.trim().split(/ /);
    const [expr, rest] = parseExprIter(tokens);
    if (rest.length > 0) {
        throw new Error('Excess token');
    }
    return expr;
}

export function parseEnvironment(code: string): Environment {
    const env = newStandardEnvironment();
    const lines = code.split(/\n/);
    for (const line of lines) {
        const [name, tokens] = line.split(/ = /);
        const expr = parseExpr(tokens);
        env.set(name, expr);
    }
    return env;
}
