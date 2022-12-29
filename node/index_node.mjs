import { tests, format } from './entry.mjs';
import { hrtime } from 'node:process';

const NS_PER_SEC = 1e9;
function to_nano(time) {
    return (time[0] * NS_PER_SEC + time[1]);
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
            
            results.push(to_nano(end));
        }

        results.sort((a, b) => a - b);
        const result = results[Math.floor(results.length / 2)];
        log(`${name} took ${format(result)} on average`);
    }
    else {
        log(`Couldn't find test ${name}`);
    }
}

const args = process.argv.slice(2);

if (args.length > 0) {
    measure(args[0]);
}
