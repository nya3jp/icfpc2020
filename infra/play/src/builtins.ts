import {
    Environment,
    Expr, isNil,
    makeApply,
    makeBoolean,
    makeList,
    makePicture,
    makeReference,
    Point,
    Value,
    debugString, makeSideEffect
} from './data';
import {evaluate} from './eval';
import {debugListString, makeNumber} from './data';
import {demodulate, modulate} from './modem';
import {getApiKey} from './auth';
import {appendLog, appendSendLog} from './logs';

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

// #32
function builtinDraw(env: Environment, a: Expr): Expr {
    const value = evaluate(env, a);
    const points: Array<Point> = [];
    for (let cur: Value = value; !isNil(env, cur); cur = evaluate(env, makeApply(makeReference('cdr'), cur))) {
        const car = evaluate(env, makeApply(makeReference('car'), cur));
        const x = evaluate(env, makeApply(makeReference('car'), car));
        const y = evaluate(env, makeApply(makeReference('cdr'), car));
        if (x.kind !== 'number' || y.kind !== 'number') {
            throw new Error('Not a number');
        }
        points.push({x: Number(x.number), y: Number(y.number)});
    }
    return makePicture(points);
}

// #33
function builtinCheckerboard(env: Environment, a: Expr, b: Expr): Expr {
    const va = evaluate(env, a);
    if (va.kind !== 'number') {
        throw new Error('Not a number');
    }
    const n = va.number;
    const points: Array<Point> = [];
    for (let x = 0; x < n; x++) {
        for (let y = 0; y < n; y++) {
            if ((x + y) % 2 === 0) {
                points.push({x, y});
            }
        }
    }
    return makePicture(points);
}

// #34
function builtinMultipledraw(env: Environment, a: Expr): Expr {
    const pa = evaluate(env, a);
    if (isNil(env, pa)) {
        return makeReference('nil');
    }
    return (
        makeApply(
            makeApply(
                makeReference('cons'),
                makeApply(
                    makeReference('draw'),
                    makeApply(
                        makeReference('car'),
                        pa))),
            makeApply(
                makeReference('multipledraw'),
                makeApply(
                    makeReference('cdr'),
                    pa))));
}

// #36
function builtinSend(env: Environment, a: Expr): Expr {
    const pa = evaluate(env, a);
    const req = modulate(env, pa);
    // Synchronous XHR - don't do this at home.
    const xhr = new XMLHttpRequest();
    xhr.open('POST', 'https://icfpc2020-api.testkontur.ru/aliens/send?apiKey=' + getApiKey(), false);
    xhr.setRequestHeader('Accept', '*/*');
    xhr.setRequestHeader('Content-Type', 'text/plain');
    xhr.send(req);
    if (xhr.status !== 200) {
        throw new Error(`HTTP ${xhr.status}`);
    }
    const res = xhr.responseText;
    let dem_req = debugListString(env, demodulate(req.trim()));
    let dem_res = debugListString(env, demodulate(res.trim()));
    appendLog(`send: ${dem_req} => ${dem_res}`);

    let sendReqExpr = demodulate(req.trim());
    let sendResExpr = demodulate(res.trim());
    appendSendLog(sendReqExpr, sendResExpr);
    return makeSideEffect(demodulate(res.trim()));
}

// #37
function builtinIf0(env: Environment, a: Expr): Expr {
    const v = evaluate(env, a);
    if (v.kind !== 'number') {
        throw new Error('Not a number');
    }
    return makeBoolean(v.number === BigInt(0));
}

// ap ap f38 x2 x0 = ap ap ap ifzero ap car x0 ( ap modem ap car ap cdr x0 , ap multipledraw ap car ap cdr ap cdr x0 ) ap ap ap interact x2 ap modem ap car ap cdr x0 ap send ap car ap cdr ap cdr x0
function f38(env: Environment, x2: Expr, x0: Expr): Expr {
    return (
        makeApply(
            makeApply(
                makeApply(
                    makeReference('if0'),
                    makeApply(
                        makeReference('car'),
                        x0)),
                makeList([
                    makeApply(
                        makeReference('car'),
                        makeApply(
                            makeReference('cdr'),
                            x0)),
                    makeApply(
                        makeReference('multipledraw'),
                        makeApply(
                            makeReference('car'),
                            makeApply(
                                makeReference('cdr'),
                                makeApply(
                                    makeReference('cdr'),
                                    x0)))),
                ])),
            makeApply(
                makeApply(
                    makeApply(
                        makeReference('interact'),
                        x2),
                    makeApply(
                        makeReference('car'),
                        makeApply(
                            makeReference('cdr'),
                            x0))),
                makeApply(
                    makeReference('send'),
                    makeApply(
                        makeReference('car'),
                        makeApply(
                            makeReference('cdr'),
                            makeApply(
                                makeReference('cdr'),
                                x0)))))));
}

// ap ap ap interact x2 x4 x3 = ap ap f38 x2 ap ap x2 x4 x3
function interact(env: Environment, x2: Expr, x4: Expr, x3: Expr): Expr {
    return makeApply(makeApply(makeReference('f38'), x2), makeApply(makeApply(x2, x4), x3));
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
    register('draw', func1Value(builtinDraw));
    register('checkerboard', func2Value(builtinCheckerboard));
    register('multipledraw', func1Value(builtinMultipledraw));
    register('send', func1Value(builtinSend));
    register('if0', func1Value(builtinIf0));
    register('f38', func2Value(f38));
    register('interact', func3Value(interact));
    return env;
}
