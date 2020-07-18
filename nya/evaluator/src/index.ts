import {parseEnvironment, parseExpr} from './parser';
import {galaxyDefs} from './galaxy';
import {evaluate} from './eval';
import {debugString} from './data';

const env = parseEnvironment(galaxyDefs);

const main = parseExpr('ap ap ap interact galaxy nil ap ap cons 0 0');
const value = evaluate(env, main);
console.log(`${debugString(env, value)}`);
