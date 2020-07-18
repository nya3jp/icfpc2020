import {PictureValue, Point} from './data';

export function renderPicture(pic: PictureValue, ch: string): string {
    const INF = 100000000;
    let minX = INF, minY = INF, maxX = -INF, maxY = -INF;
    for (const p of pic.points) {
        minX = Math.min(minX, p.x);
        minY = Math.min(minY, p.y);
        maxX = Math.max(maxX, p.x);
        maxY = Math.max(maxY, p.y);
    }
    if (minX >= maxX) {
        return '';
    }

    function key(p: Point): number {
        return p.x * 1000000 + p.y;
    }

    const pointSet = new Set<number>();
    for (const p of pic.points) {
        pointSet.add(key(p));
    }

    const chars = [];
    for (let y = minY; y <= maxY; ++y) {
        for (let x = minX; x <= maxX; ++x) {
            chars.push(pointSet.has(key({x, y})) ? ch : ' ');
        }
        chars.push('\n');
    }
    return chars.join('');
}
