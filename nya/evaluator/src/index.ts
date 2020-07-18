import {parseEnvironment, parseExpr} from './parser';
import {galaxyDefs} from './galaxy';
import {evaluate} from './eval';
import {debugString, makeApply, parseList, PictureValue} from './data';
import {renderPicture} from './picture';

const env = parseEnvironment(galaxyDefs);
const main = parseExpr('ap interact galaxy');

let state = parseExpr('nil');
let point = parseExpr('ap ap cons 0 0');
for (let i = 0; i < 100; i++) {
    console.log(`State: ${debugString(env, state)}`);
    const result = evaluate(env, makeApply(makeApply(main, state), point));
    const [newState, picValues] = parseList(env, result);
    const pics = parseList(env, picValues);
    for (const pic of pics) {
        const rendered = renderPicture(pic as PictureValue, '#');
        console.log(rendered);
    }
    state = newState;
}
