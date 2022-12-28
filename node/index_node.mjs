import { run as leet_9 } from './leet_9/index.mjs';
import { hrtime } from 'node:process';

const tests = {
    leet_9
};

const NS_PER_SEC = 1e9;
function to_milli(time) {
    return (time[0] * NS_PER_SEC + time[1]) / 1e6;
}

function measure(name) {
    const log = console.log.bind(console);

    // disable logging in tests
    console.log = () => {};

    if (name in tests) {
        const results = [];

        for (let i = 0; i < 100; i++) {
            const start = hrtime();
            tests[name]();
            const end = hrtime(start);
            
            results.push(to_milli(end));
        }

        results.sort((a, b) => a - b);
        const result = results[Math.floor(results.length / 2)];
        log(`${name} took ${result}ms on average`);
    }
    else {
        log(`Couldn't find test ${name}`);
    }
}

const args = process.argv.slice(2);

if (args.length > 0) {
    measure(args[0]);
}