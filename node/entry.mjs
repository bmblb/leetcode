import { run as leet_2 } from './leet_2/index.mjs';
import { run as leet_9 } from './leet_9/index.mjs';

export const tests = {
    leet_2,
    leet_9
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