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

import {Point, StaticData} from './data';

export interface Picture {
    points: Array<Point>
}

export function parsePicture(data: StaticData): Picture {
    if (data.dataType !== 'list') {
        throw new Error('Not a list');
    }
    const points: Array<Point> = [];
    for (const elem of data.elems) {
        if (elem.dataType !== 'cons') {
            throw new Error('Not a cons');
        }
        if (elem.car.dataType !== 'number' || elem.cdr.dataType !== 'number') {
            throw new Error('Not a number');
        }
        points.push({x: Number(elem.car.number), y: Number(elem.cdr.number)});
    }
    return {points};
}

export function parsePictures(data: StaticData): Array<Picture> {
    if (data.dataType !== 'list') {
        throw new Error('Not a list');
    }
    return data.elems.map(parsePicture);
}
