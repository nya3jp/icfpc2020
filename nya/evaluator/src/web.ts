import {parseExpr} from './parser';
import {newGalaxyEnvironment} from './galaxy';
import {evaluate} from './eval';
import {
    debugString,
    makeApply, makeNumber,
    makeReference,
    parseList,
    PictureValue,
    Point
} from './data';

const env = newGalaxyEnvironment();
const main = parseExpr('ap interact galaxy');

let state = parseExpr('nil');
let point = {x: 0, y: 0};

const canvasElem = document.getElementById('canvas') as HTMLCanvasElement;
const stateElem = document.getElementById('state') as HTMLElement;
const pointElem = document.getElementById('point') as HTMLElement;

const VIEW_MARGIN = 60;

interface View {
    minX: number
    minY: number
    maxX: number
    maxY: number
}

const DEFAULT_VIEW = {
    minX: 0,
    minY: 0,
    maxX: 1,
    maxY: 1,
}

let lastView = DEFAULT_VIEW;

function clearCanvas(): void {
    const ctx = canvasElem.getContext('2d');
    if (!ctx) {
        throw new Error('Canvas context unavailable');
    }

    ctx.fillStyle = 'black';
    ctx.fillRect(0, 0, canvasElem.width, canvasElem.height);
}

function renderCanvas(pics: Array<PictureValue>): void {
    const INF = 100000000;
    let view = {
        minX: INF,
        minY: INF,
        maxX: -INF,
        maxY: -INF,
    }
    for (const pic of pics) {
        for (const p of pic.points) {
            view.minX = Math.min(view.minX, p.x);
            view.minY = Math.min(view.minY, p.y);
            view.maxX = Math.max(view.maxX, p.x + 1);
            view.maxY = Math.max(view.maxY, p.y + 1);
        }
    }
    if (view.minX >= view.maxX) {
        view = DEFAULT_VIEW
    }

    const ctx = canvasElem.getContext('2d');
    if (!ctx) {
        throw new Error('Canvas context unavailable');
    }

    ctx.fillStyle = 'black';
    ctx.fillRect(0, 0, canvasElem.width, canvasElem.height);

    const d = Math.min((canvasElem.width - VIEW_MARGIN) / (view.maxX - view.minX), (canvasElem.height - VIEW_MARGIN) / (view.maxY - view.minY));
    const ox = (canvasElem.width - d * (view.maxX - view.minX)) / 2;
    const oy = (canvasElem.height - d * (view.maxY - view.minY)) / 2;
    function translate(p: Point): Point {
        return {x: ox + d * (p.x - view.minX), y: oy + d * (p.y - view.minY)};
    }
    for (let i = 0; i < pics.length; ++i) {
        const pic = pics[i];
        ctx.fillStyle = `hsla(${360 * i / pics.length}, 100%, 50%, 0.5)`;
        for (const p of pic.points) {
            const q = translate(p);
            ctx.fillRect(q.x, q.y, d, d);
        }
    }

    lastView = view;
}

function updateStates(): void {
    stateElem.textContent = debugString(env, state);
    pointElem.textContent = `(${point.x}, ${point.y})`;
}

function step() {
    const pt = makeApply(makeApply(makeReference('cons'), makeNumber(point.x)), makeNumber(point.y));
    const result = evaluate(env, makeApply(makeApply(main, state), pt));
    const [newState, picValues] = parseList(env, result);
    const pics = parseList(env, picValues) as Array<PictureValue>;
    renderCanvas(pics);
    updateStates();
    state = newState;
}

function onClickCanvas(ev: MouseEvent) {
    const view = lastView;
    const d = Math.min((canvasElem.width - VIEW_MARGIN) / (view.maxX - view.minX), (canvasElem.height - VIEW_MARGIN) / (view.maxY - view.minY));
    const ox = (canvasElem.width - d * (view.maxX - view.minX)) / 2;
    const oy = (canvasElem.height - d * (view.maxY - view.minY)) / 2;
    point = {x: Math.floor((ev.offsetX - ox) / d + view.minX), y: Math.floor((ev.offsetY - oy) / d + view.minY)};
    step();
}

clearCanvas();
updateStates();

canvasElem.addEventListener('click', onClickCanvas);

interface Window {
    step(): void
}
declare var window: Window;
window.step = step;
