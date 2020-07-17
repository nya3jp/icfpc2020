import {Environment, Expr, Value} from './data';

export function evaluate(env: Environment, expr: Expr): Value {
    switch (expr.kind) {
        case 'apply':
            const func = evaluate(env, expr.lhs);
            if (func.kind !== 'func') {
                throw new Error(`Invalid function call: ${func.kind}`);
            }
            return evaluate(env, func.func(env, expr.rhs));
        case 'reference':
            const expr2 = env.get(expr.name);
            if (!expr2) {
                throw new Error(`Undefined reference: ${expr.name}`);
            }
            return evaluate(env, expr2);
        case 'literal':
            return expr.value;
    }
}
