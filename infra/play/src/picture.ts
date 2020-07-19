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
