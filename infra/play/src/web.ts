import {parseExpr} from './parser';
import {newGalaxyEnvironment} from './galaxy';
import {evaluate} from './eval';
import {
    debugString, Expr,
    makeApply, makeNumber,
    makeReference,
    parseList,
    PictureValue,
    Point
} from './data';
import {getLogs} from './logs';

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
const pixelSizeElem = document.getElementById('fixed') as HTMLInputElement;
const infoElem = document.getElementById('info') as HTMLElement;
const logsElem = document.getElementById('logs') as HTMLTextAreaElement;

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
}

function updateUI(): void {
    const { state, point, pics } = history[historyPos];
    renderCanvas(pics);
    infoElem.textContent = `Step ${historyPos + 1} / ${history.length} | Last point (${point.x}, ${point.y})`;
    stateElem.value = debugString(env, state);
    logsElem.textContent = getLogs().reverse().join('\n');
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

function reportError(e: Error): void {
    alert(e);
    throw e;
}

function init(): void {
    canvasElem.addEventListener('click', onClickCanvas);
    stateElem.addEventListener('change', onStateChanged);
    pixelSizeElem.addEventListener('change', onPixelSizeChanged);
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
