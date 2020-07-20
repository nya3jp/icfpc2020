import {Point, PrettyData} from './data';

export interface Picture {
    points: Array<Point>
}

export function parsePicture(data: PrettyData): Picture {
    if (data.kind !== 'list') {
        throw new Error('Not a list');
    }
    const points: Array<Point> = [];
    for (const elem of data.elems) {
        if (elem.kind !== 'cons') {
            throw new Error('Not a cons');
        }
        if (elem.car.kind !== 'number' || elem.cdr.kind !== 'number') {
            throw new Error('Not a number');
        }
        points.push({x: Number(elem.car.number), y: Number(elem.cdr.number)});
    }
    return {points};
}

export function parsePictures(data: PrettyData): Array<Picture> {
    if (data.kind !== 'list') {
        throw new Error('Not a list');
    }
    return data.elems.map(parsePicture);
}

export function pictureEqual(a: Picture, b: Picture): boolean {
    if (a.points.length !== b.points.length) {
        return false;
    }
    for (let i = 0; i < a.points.length; i++) {
        const pa = a.points[i];
        const pb = b.points[i];
        if (pa.x !== pb.x || pa.y !== pb.y) {
            return false;
        }
    }
    return true;
}

export function picturesEqual(a: Array<Picture>, b: Array<Picture>): boolean {
    if (a.length !== b.length) {
        return false;
    }
    for (let i = 0; i < a.length; i++) {
        if (!pictureEqual(a[i], b[i])) {
            return false;
        }
    }
    return true;
}
