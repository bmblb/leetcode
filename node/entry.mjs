import { run as leet_2 } from './leet_2/index.mjs';
import { run as leet_9 } from './leet_9/index.mjs';
import { run as leet_13 } from './leet_13/index.mjs';
import { run as leet_14 } from './leet_14/index.mjs';
import { run as leet_20 } from './leet_20/index.mjs';
import { run as leet_21 } from './leet_21/index.mjs';
import { run as leet_26 } from './leet_26/index.mjs';
import { run as leet_27 } from './leet_27/index.mjs';
import { run as leet_35 } from './leet_35/index.mjs';

export const tests = {
    leet_2,
    leet_9,
    leet_13,
    leet_14,
    leet_20,
    leet_21,
    leet_26,
    leet_27,
    leet_35,
};

export function format(value) {
    const units = ['ns', 'μs', 'ms', 's'];

    let output = [];

    for (let name of units) {
        const val = value % 1000;
        output.push([name, val]);
        value = Math.floor(value / 1000);
    }

    return output.filter(([_, val]) => val > 0).reverse().map(([unit, val]) => `${val}${unit}`).join(' ');
}
