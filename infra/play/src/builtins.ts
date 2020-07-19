import {
    Environment,
    Expr,
    makeApply,
    makeBoolean,
    makeReference,
    Value,
} from './data';
import {evaluate} from './eval';
import {makeNumber} from './data';

function func1Value(f: (env: Environment, a: Expr) => Expr): Value {
    return {kind: 'func', func: f};
}

function func2Value(f: (env: Environment, a: Expr, b: Expr) => Expr): Value {
    return {kind: 'func', func: (env: Environment, a: Expr) => func1Value((env, b) => f(env, a, b))};
}

function func3Value(f: (env: Environment, a: Expr, b: Expr, c: Expr) => Expr): Value {
    return {kind: 'func', func: (env: Environment, a: Expr) => func2Value((env, b, c) => f(env, a, b, c))};
}

// #5
function builtinInc(env: Environment, a: Expr): Expr {
    const va = evaluate(env, a);
    if (va.kind !== 'number') {
        throw new Error('not a number')
    }
    return makeNumber(va.number + BigInt(1));
}

// #6
function builtinDec(env: Environment, a: Expr): Expr {
    const va = evaluate(env, a);
    if (va.kind !== 'number') {
        throw new Error('not a number')
    }
    return makeNumber(va.number - BigInt(1));
}

// #7
function builtinAdd(env: Environment, a: Expr, b: Expr): Expr {
    const va = evaluate(env, a);
    const vb = evaluate(env, b);
    if (va.kind !== 'number' || vb.kind !== 'number') {
        throw new Error('not a number')
    }
    return makeNumber(va.number + vb.number);
}

// #9
function builtinMul(env: Environment, a: Expr, b: Expr): Expr {
    const va = evaluate(env, a);
    const vb = evaluate(env, b);
    if (va.kind !== 'number' || vb.kind !== 'number') {
        throw new Error('not a number')
    }
    return makeNumber(va.number * vb.number);
}

// #10
function builtinDiv(env: Environment, a: Expr, b: Expr): Expr {
    const va = evaluate(env, a);
    const vb = evaluate(env, b);
    if (va.kind !== 'number' || vb.kind !== 'number') {
        throw new Error('not a number')
    }
    return makeNumber(va.number / vb.number);
}

// #11
function builtinEq(env: Environment, a: Expr, b: Expr): Expr {
    const va = evaluate(env, a);
    const vb = evaluate(env, b);
    if (va.kind !== 'number' || vb.kind !== 'number') {
        throw new Error('not a number')
    }
    return makeBoolean(va.number === vb.number);
}

// #12
function builtinLt(env: Environment, a: Expr, b: Expr): Expr {
    const va = evaluate(env, a);
    const vb = evaluate(env, b);
    if (va.kind !== 'number' || vb.kind !== 'number') {
        throw new Error('not a number')
    }
    return makeBoolean(va.number < vb.number);
}

// #16
function builtinNeg(env: Environment, a: Expr): Expr {
    const va = evaluate(env, a);
    if (va.kind !== 'number') {
        throw new Error('not a number')
    }
    return makeNumber(-va.number);
}

// #18
function builtinS(env: Environment, a: Expr, b: Expr, c: Expr): Expr {
    return makeApply(makeApply(a, c), makeApply(b, c))
}

// #19
function builtinC(env: Environment, a: Expr, b: Expr, c: Expr): Expr {
    return makeApply(makeApply(a, c), b)
}

// #20
function builtinB(env: Environment, a: Expr, b: Expr, c: Expr): Expr {
    return makeApply(a, makeApply(b, c))
}

// #21
function builtinTrue(env: Environment, a: Expr, b: Expr): Expr {
    return a
}

// #22
function builtinFalse(env: Environment, a: Expr, b: Expr): Expr {
    return b
}

// #24
function builtinI(env: Environment, a: Expr): Expr {
    return a
}

// #25
function builtinCons(env: Environment, car: Expr, cdr: Expr, t: Expr): Expr {
    return makeApply(makeApply(t, car), cdr);
}

// #26
function builtinCar(env: Environment, a: Expr): Expr {
    return makeApply(a, makeReference('t'));
}

// #27
function builtinCdr(env: Environment, a: Expr): Expr {
    return makeApply(a, makeReference('f'));
}

// #28
export function builtinNil(env: Environment, a: Expr): Expr {
    return makeBoolean(true);
}

// #28/29
function builtinIsnilHelper(env: Environment, a: Expr, b: Expr): Expr {
    return makeBoolean(false);
}

// #29
function builtinIsnil(env: Environment, a: Expr): Expr {
    return makeApply(a, makeReference('_isnil_helper'))
}

export function newStandardEnvironment(): Environment {
    const env = new Map<string, Expr>();
    function register(name: string, value: Value) {
        env.set(name, value);
    }
    register('inc', func1Value(builtinInc));
    register('dec', func1Value(builtinDec));
    register('add', func2Value(builtinAdd));
    register('mul', func2Value(builtinMul));
    register('div', func2Value(builtinDiv));
    register('eq', func2Value(builtinEq));
    register('lt', func2Value(builtinLt));
    register('neg', func1Value(builtinNeg));
    register('s', func3Value(builtinS));
    register('c', func3Value(builtinC));
    register('b', func3Value(builtinB));
    register('t', func2Value(builtinTrue));
    register('f', func2Value(builtinFalse));
    register('i', func1Value(builtinI));
    register('cons', func3Value(builtinCons));
    register('car', func1Value(builtinCar));
    register('cdr', func1Value(builtinCdr));
    register('nil', func1Value(builtinNil));
    register('_isnil_helper', func2Value(builtinIsnilHelper));
    register('isnil', func1Value(builtinIsnil));
    return env;
}
