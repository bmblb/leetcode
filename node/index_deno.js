import { tests } from './entry.mjs';

const NS_PER_SEC = 1e9;
function to_milli(time) {
    return (time[0] * NS_PER_SEC + time[1]) / 1e6;
}

function measure(name) {
    const log = console.log.bind(console);

    // disable logging in tests
    console.log = () => {};

    if (name in tests) {
        let results = [];

        for (let i = 0; i < 100; i++) {
            const start = performance.now();
            tests[name]();
            const end = performance.now();
            
            results.push(end - start);
        }

        results = results.map(x => Math.floor(x * 10000) / 10000);

        results.sort((a, b) => a - b);
        const result = results[Math.floor(results.length / 2)];
        log(`${name} took ${result}ms on average`);
    }
    else {
        log(`Couldn't find test ${name}`);
    }
}

const args = Deno.args;

if (args.length > 0) {
    measure(args[0]);
}
