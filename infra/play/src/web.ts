import {parseExpr} from './parser';
import {newGalaxyEnvironment} from './galaxy';
import {evaluate} from './eval';
import {
    debugString,
    debugListString,
    Expr,
    makeApply,
    makeNumber,
    makeReference,
    parseList,
    Point,
    PrettyData,
    NumberData,
    prettyDataString,
    valueToPrettyData
} from './data';

import {appendSendLog, getSendLogs} from './logs';
import { annotate } from './annotate';
import { sendToServer } from './utils';
import { demodulate, modulate } from './modem';
import {parsePictures, Picture} from './picture';

const env = newGalaxyEnvironment();
const galaxyExpr = makeReference('galaxy');

interface RenderState {
    input: Expr
    state: Expr
    pics: Array<Picture>
}

const FARAWAY_POINT = {x: -10000, y: -10000};

const history: Array<RenderState> = [];
let historyPos = -1;

const canvasElem = document.getElementById('canvas') as HTMLCanvasElement;
const stateElem = document.getElementById('state') as HTMLInputElement;
const stateListElem = document.getElementById('state-list') as HTMLLabelElement;
const pixelSizeElem = document.getElementById('fixed') as HTMLInputElement;
const infoElem = document.getElementById('info') as HTMLElement;
const logsElem = document.getElementById('logs') as HTMLTextAreaElement;
const sendLogsElem = document.getElementById('sendlogs') as HTMLElement;
const annotateElem = document.getElementById('annotate') as HTMLInputElement;
const replayElem = document.getElementById('replay-player-key') as HTMLInputElement;

const VIEW_MARGIN = 60;

interface View {
    minX: number
    minY: number
    maxX: number
    maxY: number
}

function getQueryParams(key: string) {
    const temp: any = window;
    const urlParams = new URLSearchParams(temp.location.search);
    return urlParams.get(key);
}

function getPixelSize(pics: Array<Picture>): number {
    const view = computeView(pics)
    if (pixelSizeElem.value != '') {
        return parseInt(pixelSizeElem.value);
    }
    return Math.min((canvasElem.width - VIEW_MARGIN) / (view.maxX - view.minX), (canvasElem.height - VIEW_MARGIN) / (view.maxY - view.minY));
}

function computeView(pics: Array<Picture>): View {
    const INF = 100000000;
    let minX = INF, minY = INF, maxX = -INF, maxY = -INF;
    for (const pic of pics) {
        for (const p of pic.points) {
            minX = Math.min(minX, p.x);
            minY = Math.min(minY, p.y);
            maxX = Math.max(maxX, p.x + 1);
            maxY = Math.max(maxY, p.y + 1);
        }
    }
    if (minX >= maxX) {
        minX = minY = 0;
        maxX = maxY = 1;
    }
    return {minX, minY, maxX, maxY};
}

function renderCanvas(pics: Array<Picture>): void {
    const view = computeView(pics);

    const ctx = canvasElem.getContext('2d');
    if (!ctx) {
        throw new Error('Canvas context unavailable');
    }

    ctx.fillStyle = 'white';
    ctx.fillRect(0, 0, canvasElem.width, canvasElem.height);

    const d = getPixelSize(pics)
    const ox = (canvasElem.width - d * (view.maxX - view.minX)) / 2;
    const oy = (canvasElem.height - d * (view.maxY - view.minY)) / 2;
    function translate(p: Point): Point {
        return {x: ox + d * (p.x - view.minX), y: oy + d * (p.y - view.minY)};
    }
    for (let i = 0; i < pics.length; ++i) {
        const pic = pics[i];
        ctx.fillStyle = `hsla(${360 * i / pics.length}, 100%, 50%, 0.7)`;
        for (const p of pic.points) {
            const q = translate(p);
            ctx.fillRect(q.x, q.y, d, d);
        }
    }

    // annotate
    if (annotateElem.checked) {
        for (const a of annotate(view.minX, view.minY, view.maxX, view.maxY, pics)) {
            ctx.fillStyle = 'black';
            const q = translate({x:a.x, y:a.y});
            ctx.fillText(a.txt, q.x, q.y, (d*a.n)*2);
        }
    }
}

function updateUI(): void {
    const { state, pics } = history[historyPos];
    renderCanvas(pics);
    infoElem.textContent = `Step ${historyPos + 1} / ${history.length}`;
    stateElem.value = debugString(env, state);
    stateListElem.textContent = prettyDataString(valueToPrettyData(env, evaluate(env, state)));

    updateLogs();
}

const EXPLAINS: {[key: string]: string} = {
    ['req:0/0']: 'Ping',
    ['req:1/0']: 'Tutorial',
    ['req:1/1']: 'ID',
    ['req:2/0']: 'Join',
    ['req:2/2']: 'Params',
    ['req:3/0']: 'Start',
    ['req:4/0']: 'Commands',
    ['req:4/2']: 'Commands',
    ['req:4/2/[0-9]/<0>0']: 'Accelerate',
    ['req:4/2/[0-9]/<0>2']: 'Vector',
    ['req:4/2/[0-9]/<1>0']: 'Detonate',
    ['req:4/2/[0-9]/<2>0']: 'Shoot',
    ['req:4/2/[0-9]/<2>2']: 'Target',
    ['req:4/2/[0-9]/<2>3']: 'x3',
    ['req:4/2/[0-9]/<[0-2]>1']: 'ShipID',
    ['req:[2-4]/1']: 'Key',
    ['res:[0-5]:0/0']: 'Error',
    ['res:[0-5]:1/0']: 'OK',
    ['res:0:1/1']: 'Time',
    ['res:1:1/1/0/1']: 'Key',
    ['res:[2-4]:1/1']: 'Stage',
    ['res:[2-4]:1/2']: 'GameInfo',
    ['res:[2-4]:1/2/0']: 'TotalTurns',
    ['res:[2-4]:1/2/1']: 'Role',
    ['res:[2-4]:1/2/2']: 'x2',
    ['res:[2-4]:1/2/3']: 'x3',
    ['res:[2-4]:1/2/4']: 'x4',
    ['res:[2-4]:1/3/0']: 'Tick',
    ['res:[2-4]:1/3/1']: 'x1',
    ['res:[2-4]:1/3/2']: 'Ships',
    ['res:[2-4]:1/3/2/[0-9]/0']: 'Ship#',
    ['res:[2-4]:1/3/2/[0-9]/0/0']: 'Role',
    ['res:[2-4]:1/3/2/[0-9]/0/1']: 'ID',
    ['res:[2-4]:1/3/2/[0-9]/0/2']: 'Pos',
    ['res:[2-4]:1/3/2/[0-9]/0/3']: 'Velocity',
    ['res:[2-4]:1/3/2/[0-9]/0/4']: 'Params',
    ['res:[2-4]:1/3/2/[0-9]/0/4/0']: 'Energy',
    ['res:[2-4]:1/3/2/[0-9]/0/4/1']: 'LaserPower',
    ['res:[2-4]:1/3/2/[0-9]/0/4/2']: 'CoolDownPerTurn',
    ['res:[2-4]:1/3/2/[0-9]/0/4/3']: 'Life',
    ['res:[2-4]:1/3/2/[0-9]/0/5']: 'Heat',
    ['res:[2-4]:1/3/2/[0-9]/0/6']: 'x6',
    ['res:[2-4]:1/3/2/[0-9]/0/7']: 'x7',
    ['res:[2-4]:1/3/2/[0-9]/1']: 'Commands#',
    ['res:[2-4]:1/3/2/[0-9]/1/[0-9]/<0>0']: 'Accelerate',
    ['res:[2-4]:1/3/2/[0-9]/1/[0-9]/<0>2']: 'Vector',
    ['res:[2-4]:1/3/2/[0-9]/1/[0-9]/<1>0']: 'Detonate',
    ['res:[2-4]:1/3/2/[0-9]/1/[0-9]/<2>0']: 'Shoot',
    ['res:[2-4]:1/3/2/[0-9]/1/[0-9]/<2>2']: 'Target',
    ['res:[2-4]:1/3/2/[0-9]/1/[0-9]/<2>3']: 'x3',
    ['res:[2-4]:1/3/2/[0-9]/1/[0-9]/<[0-2]>1']: 'ShipID',
}

function updateLogs(): void {
    type TreePos = string | null;

    function emph(s: string): string {
        return `<b>${s}</b>`;
    }
    function pushPos(pos: TreePos, elems: Array<PrettyData>, i: number): TreePos {
        if (!pos) {
            return null;
        }
        if (/^req:4\/2\/[0-9]$/.test(pos) ||
            /^res:[2-4]:1\/3\/2\/[0-9]\/1\/[0-9]$/.test(pos)) {
            return `${pos}/<${(elems[0] as NumberData).number}>${i}`
        }
        return `${pos}/${i}`;
    }
    function explain(data: PrettyData, pos: TreePos): string {
        if (!pos) {
            return '';
        }
        for (const key in EXPLAINS) {
            const re = new RegExp('^' + key + '$');
            if (re.test(pos)) {
                let msg = EXPLAINS[key];
                const path = pos.split(/[:/]/);
                msg = msg.replace('#', path[path.length-2]);
                return `<span class="annotation">${msg}: </span>`;
            }
        }
        return '';
    }
    function toDiffedLispList(data: PrettyData, last: PrettyData, pos: TreePos): string {
        switch (data.kind) {
            case 'number':
                const s = toLispList(data, pos);
                if (last.kind == 'number' && last.number === data.number) {
                    return s;
                }
                return emph(s);
            case 'list':
                if (last.kind !== 'list') {
                    return emph(toLispList(data, pos));
                }
                const elems: Array<string> = [];
                for (let i = 0; i < data.elems.length; ++i) {
                    const dataElem = data.elems[i];
                    const lastElem = last.elems[i];
                    if (!lastElem) {
                        elems.push(emph(toLispList(dataElem, pushPos(pos, data.elems, i))));
                    } else {
                        elems.push(toDiffedLispList(dataElem, lastElem, pushPos(pos, data.elems, i)));
                    }
                }
                return explain(data, pos) + `[${elems.join(', ')}]`;
            case 'cons':
                if (last.kind !== 'cons') {
                    return emph(toLispList(data, pos));
                }
                return explain(data, pos) + `(${toDiffedLispList(data.car, last.car, null)} . ${toDiffedLispList(data.cdr, last.cdr, null)})`;
        }
    }
    function toLispList(data: PrettyData, pos: TreePos): string {
        switch (data.kind) {
            case 'number':
                return explain(data, pos) + String(data.number);
            case 'list':
                return explain(data, pos) + `[${data.elems.map((e, i) => toLispList(e, pushPos(pos, data.elems, i))).join(', ')}]`;
            case 'cons':
                return explain(data, pos) + `(${toLispList(data.car, null)} . ${toLispList(data.cdr, null)})`;
        }
    }

    const sends = getSendLogs();

    let elems: Array<string> = [];
    for (let i = sends.length - 1; i >= 0; i--) { // new -> old
        const {req, res} = sends[i];
        const op = req.kind === 'list' && req.elems[0].kind === 'number' ? String(req.elems[0].number) : '?';
        const code = res.kind === 'list' && res.elems[0].kind === 'number' ? String(res.elems[0].number) : '?';
        let reqLog: String;
        let resLog: String;
        if (i == 0) {
            reqLog = toLispList(req, `req:${op}`);
            resLog = toLispList(res, `res:${op}:${code}`);
        } else {
            reqLog = toDiffedLispList(req, sends[i-1].req, `req:${op}`);
            resLog = toDiffedLispList(res, sends[i-1].res, `res:${op}:${code}`);
        }
        elems.push(`${reqLog} → ${resLog}`);
    }
    sendLogsElem.innerHTML = elems.join('<br>');
}

const DEBUG_INTERACT = false;

function interact(state: Expr, input: Expr): void {
    try {
        if (DEBUG_INTERACT) {
            console.log('interact start');
        }
        let pics: Array<Picture>;
        loop: while (true) {
            if (DEBUG_INTERACT) {
                console.log(`evaluate state=${prettyDataString(valueToPrettyData(env, evaluate(env, state)))} input=${prettyDataString(valueToPrettyData(env, evaluate(env, input)))}`);
            }
            const result = evaluate(env, makeApply(makeApply(galaxyExpr, state), input));
            const [syscall, s, output] = parseList(env, result);
            state = s;
            if (syscall.kind !== 'number') {
                throw new Error('Flag not a number');
            }
            switch (Number(syscall.number)) {
                case 0:  // Draw
                    pics = parsePictures(valueToPrettyData(env, output));
                    break loop;
                case 1:  // Send
                    const modReq = modulate(env, output);
                    const modRes = sendToServer(modReq)
                    input = demodulate(modRes);
                    const req = valueToPrettyData(env, output);
                    const res = valueToPrettyData(env, evaluate(env, input))
                    appendSendLog({req, res});
                    if (DEBUG_INTERACT) {
                        console.log(`send:\nstate=${debugString(env, state)}\nreq=${prettyDataString(req)}\nres=${prettyDataString(res)}`);
                    }
                    break;
                default:
                    throw new Error(`Unsupported system call ${syscall.number}`);
            }
        }
        if (DEBUG_INTERACT) {
            console.log('interact end');
        }
        history.splice(historyPos+1);
        history.push({state, input, pics});
        historyPos++;
    } catch (e) {
        reportError(e);
    } finally {
        updateUI();
    }
}

function interactPoint(state: Expr, point: Point): void {
    const input = makeApply(makeApply(makeReference('cons'), makeNumber(BigInt(point.x))), makeNumber(BigInt(point.y)));
    interact(state, input);
}

function forward(): void {
    if (historyPos + 1 >= history.length) {
        return;
    }
    historyPos++;
    updateUI();
}

function backward(): void {
    if (historyPos === 0) {
        return;
    }
    historyPos--;
    updateUI();
}

function loadState(stateStr: string, point: Point = FARAWAY_POINT): void {
    let state: Expr;
    try {
        state = parseExpr(stateStr);
    } catch (e) {
        reportError(e);
        throw new Error('not reached');
    }
    interactPoint(state, point);
}

function loadReplay(key: string): void {
    try {
        loadState(`ap ap cons 5 ap ap cons ap ap cons 4 ap ap cons ${key} ap ap cons nil ap ap cons nil ap ap cons nil ap ap cons nil ap ap cons ap ap cons 36 0 ap ap cons 21839 nil ap ap cons 9 ap ap cons nil nil`, {x: 26, y: 0});
        replayElem.value = key;
    } catch (e) {
        replayElem.value = '';
        throw e;
    }
}

function onClickCanvas(ev: MouseEvent): void {
    const { state, pics } = history[historyPos];
    const view = computeView(pics);
    const d = getPixelSize(pics);
    const ox = (canvasElem.width - d * (view.maxX - view.minX)) / 2;
    const oy = (canvasElem.height - d * (view.maxY - view.minY)) / 2;
    const point = {x: Math.floor((ev.offsetX - ox) / d + view.minX), y: Math.floor((ev.offsetY - oy) / d + view.minY)};
    interactPoint(state, point);
}

function onStateChanged(): void {
    loadState(stateElem.value.trim());
}

function onPixelSizeChanged(): void {
    updateUI();
}

function onAnnotateChanged(): void {
    updateUI();
}

function onReplayPlayerKeyChanged(): void {
    loadReplay(replayElem.value.trim());
}

function reportError(e: Error): void {
    alert(e);
    throw e;
}

function init(): void {
    canvasElem.addEventListener('click', onClickCanvas);
    stateElem.addEventListener('change', onStateChanged);
    pixelSizeElem.addEventListener('change', onPixelSizeChanged);
    annotateElem.addEventListener('change', onAnnotateChanged);
    replayElem.addEventListener('change', onReplayPlayerKeyChanged);

    const initState = parseExpr('ap ap cons 2 ap ap cons ap ap cons 1 ap ap cons -1 nil ap ap cons 0 ap ap cons nil nil');
    interactPoint(initState, FARAWAY_POINT);

    const givenState = getQueryParams('state');
    if (givenState) {
        loadState(givenState);
        return;
    }
    const givenKey = getQueryParams('key');
    if (givenKey) {
        loadReplay(givenKey);
        return;
    }
}

init();

interface Window {
    step(): void
    forward(): void
    backward(): void
}
declare var window: Window;
window.forward = forward;
window.backward = backward;
