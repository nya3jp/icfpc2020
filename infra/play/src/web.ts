import {parseExpr} from './parser';
import {newGalaxyEnvironment} from './galaxy';
import {evaluate} from './eval';
import {
    debugString, debugListString, Expr, isNil,
    makeApply, makeNumber,
    makeReference,
    parseList,
    PictureValue,
    Point, PrettyData, NumberData
} from './data';

import {
    builtinNil
} from './builtins';

import {getSendLogs} from './logs';
import { annotate } from './annotate';
import { sendToServer } from './utils';
import { demodulate, modulate } from './modem';

const env = newGalaxyEnvironment();
const mainExpr = parseExpr('ap interact galaxy');

interface State {
    state: Expr
    point: Point
    pics: Array<PictureValue>
}

const FAWAWAY_POINT = {x: -10000, y: -10000};
const initState = {
    state: parseExpr('ap ap cons 2 ap ap cons ap ap cons 1 ap ap cons -1 nil ap ap cons 0 ap ap cons nil nil'),
    point: FAWAWAY_POINT,
    pics: [],
}

const history: Array<State> = [initState];
let historyPos = 0;

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

function getPixelSize(pics: Array<PictureValue>): number {
    const view = computeView(pics)
    if (pixelSizeElem.value != '') {
        return parseInt(pixelSizeElem.value);
    }
    return Math.min((canvasElem.width - VIEW_MARGIN) / (view.maxX - view.minX), (canvasElem.height - VIEW_MARGIN) / (view.maxY - view.minY));
}

function computeView(pics: Array<PictureValue>): View {
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

function renderCanvas(pics: Array<PictureValue>): void {
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
    const { state, point, pics } = history[historyPos];
    renderCanvas(pics);
    infoElem.textContent = `Step ${historyPos + 1} / ${history.length} | Last point (${point.x}, ${point.y})`;
    stateElem.value = debugString(env, state);
    stateListElem.textContent = debugListString(env, state);

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

function interact(state: Expr, point: Point): void {
    const pt = makeApply(makeApply(makeReference('cons'), makeNumber(BigInt(point.x))), makeNumber(BigInt(point.y)));
    try {
        const result = evaluate(env, makeApply(makeApply(mainExpr, state), pt));
        const [newState, picValues] = parseList(env, result);
        const pics = parseList(env, picValues) as Array<PictureValue>;
        history.splice(historyPos+1);
        history.push({state: newState, point: point, pics});
        historyPos++;
    } catch (e) {
        reportError(e);
    }
    updateUI();
}

function step(): void {
    const { point, state } = history[historyPos];
    interact(state, point);
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

function onClickCanvas(ev: MouseEvent): void {
    const { state, pics } = history[historyPos];
    const view = computeView(pics);
    const d = getPixelSize(pics);
    const ox = (canvasElem.width - d * (view.maxX - view.minX)) / 2;
    const oy = (canvasElem.height - d * (view.maxY - view.minY)) / 2;
    const point = {x: Math.floor((ev.offsetX - ox) / d + view.minX), y: Math.floor((ev.offsetY - oy) / d + view.minY)};
    interact(state, point);
}

function onStateChanged(ev: Event): void {
    try {
        const state = parseExpr(stateElem.value.trim());
        interact(state, FAWAWAY_POINT);
    } catch(e) {
        updateUI();
        reportError(e);
    }
}

function onPixelSizeChanged(ev: Event): void {
    updateUI();
}

function onAnnotateChanged(ev: Event): void {
    updateUI();
}

function onReplayPlayerKeyChanged(ev: Event): void {
    function cadr(expr: string): string {
        return `ap car ap cdr ${expr}`
    }
    function replayState(key: string, history: string, timestamp: string): string {
        return `ap ap cons 5 ap ap cons ap ap cons 7 ap ap cons ${key} ap ap `
            + `cons nil ap ap cons nil ap ap cons nil ap ap cons ${history} `
            + `ap ap cons ap ap cons 0 0 ap ap cons ${timestamp} nil ap ap `
            + `cons 1 ap ap cons nil nil`;
    }

    const playerKey = replayElem.value.trim();
    try {
        const getTimestampRequest = evaluate(env,
            parseExpr(`ap ap cons 0 nil`));
        const timestampVal = evaluate(env,
            demodulate(sendToServer(modulate(env, getTimestampRequest))));
        const timestamp = evaluate(env,
            parseExpr(cadr(debugString(env, timestampVal))));

        const getHistoryRequest = evaluate(env,
            parseExpr(`ap ap cons 5 ap ap cons ${playerKey} nil`));
        const history = evaluate(env,
            demodulate(sendToServer(modulate(env, getHistoryRequest))));
        const historyStr = debugString(env, history);
        if (historyStr === "ap ap cons 0 nil") {
            alert("Failed to fetch the history");
            return;
        }

        const state = parseExpr(
            replayState(playerKey, debugString(env, history), debugString(env, timestamp)));
        interact(state, FAWAWAY_POINT);
    } catch (e) {
        updateUI();
        reportError(e);
    }
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
    const givenState = getQueryParams('state');
    if (givenState !== null) {
        stateElem.value = givenState;
        onStateChanged(new Event('change'));
    }
    step();
}

init();

interface Window {
    step(): void
    forward(): void
    backward(): void
}
declare var window: Window;
window.step = step;
window.forward = forward;
window.backward = backward;
