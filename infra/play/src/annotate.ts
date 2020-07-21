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

import {
    Point
} from './data';
import {Picture} from './picture';

export interface Annotate {
    x: number
    y: number
    n: number
    txt: string
}

export function annotate(minX: number, minY:number, maxX:number, maxY:number, pics: Array<Picture>): Array<Annotate> {
    function translate(p: Point): Point {
        return {x:p.x-minX, y:p.y-minY};
    };

    let DX = maxX - minX + 1;
    let DY = maxY - minY + 1;

    let res:Array<Annotate> = [];
    for (let i = 0; i < pics.length; ++i) {
        const pic = pics[i];
        let img: Array<Array<boolean>> = [];
        for (let x = 0; x < DX; ++x) {
            let col: Array<boolean> = [];
            for (let y = 0; y < DY; ++y) {
                col.push(false);
            }
            img.push(col);
        }

        for (const p of pic.points) {
            const q = translate(p);
            img[q.x][q.y] = true;
        }

        function get(x: number, y:number): boolean {
            if (x<0||y<0||x>=DX||y>=DY)return false;
            return img[x][y];
        }

        // number
        for (let y = 0; y < DY-1; ++y) {
            for (let x = 0; x < DX-1; ++x) {
                if(!img[x][y] && img[x+1][y] && img[x][y+1]) {
                    let n = 1;
                    for (; x+n < DX && img[x+n][y]; n++);
                    let ok = true;
                    for (let i=0; i<n; i++) {
                        if(get(x-1,y+i)) {
                            ok = false;break;
                        }
                        if(get(x+i,y-1)) {
                            ok=false;break;
                        }
                        if(i>0 && !img[x+i][y]) {
                            ok=false;break;
                        }
                    }
                    if(get(x-1,y+n)||get(x+1,y+n)) ok=false;
                    if(get(x+n,y)||get(x+n,y-1)||get(x+n,y+1)) ok=false;
                    let neg = get(x,y+n);
                    if (!ok) {
                        continue;
                    }
                    let num = BigInt(0);
                    let idx = 0;
                    for (let yi=1; yi<n; yi++) {
                        for (let xi=1; xi<n; xi++) {
                            if (get(x+xi,y+yi)) {
                                num += BigInt(1) << BigInt(idx);
                            }
                            idx++;
                        }
                    }
                    if (neg) {
                        num *= BigInt(-1);
                    }
                    res.push(
                        {
                            x:x+minX,
                            y:y+minY,
                            n:n,
                            txt:num.toString()
                        }
                    )
                }
            }
        } 
    }

    return res;
}