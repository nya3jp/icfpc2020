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

import {newStandardEnvironment} from './builtins';
import {Environment, Expr} from './data';

function parseExprIter(tokens: Array<string>): [Expr, Array<string>] {
    const token = tokens[0];
    if (token === 'ap') {
        const [lhs, rest1] = parseExprIter(tokens.slice(1));
        const [rhs, rest2] = parseExprIter(rest1);
        return [{kind: 'apply', lhs, rhs}, rest2];
    }
    if (/^-?\d+$/.test(token)) {
        return [{kind: 'number', number: BigInt(token)}, tokens.slice(1)];
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
