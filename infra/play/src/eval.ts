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
