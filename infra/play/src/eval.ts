import {Environment, Expr, Value} from './data';

export function evaluate(env: Environment, expr: Expr): Value {
    switch (expr.kind) {
        case 'number':
        case 'func':
            return expr
        case 'apply': {
            if (expr.cache) {
                return expr.cache;
            }
            if (!expr.lhs || !expr.rhs) {
                throw new Error('Uncached apply missing LHS/RHS');
            }
            const func = evaluate(env, expr.lhs);
            if (func.kind !== 'func') {
                throw new Error(`Invalid function call: ${func.kind}`);
            }
            const value = evaluate(env, func.func(env, expr.rhs));
            expr.cache = value;
            // Release thunk trees.
            expr.lhs = undefined;
            expr.rhs = undefined;
            return value;
        }
        case 'reference': {
            if (expr.cache) {
                return expr.cache;
            }
            const expr2 = env.get(expr.name);
            if (!expr2) {
                throw new Error(`Undefined reference: ${expr.name}`);
            }
            const value = evaluate(env, expr2);
            expr.cache = value;
            return value;
        }
    }
}
