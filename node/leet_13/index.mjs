import { assert_eq } from "../utils.mjs";

function roman_to_int(s) {
    let numerals = Object.fromEntries([
        ['I', 1],
        ['V', 5],
        ['X', 10],
        ['L', 50],
        ['C', 100],
        ['D', 500],
        ['M', 1000]
    ]);

    // sub optimal, but let's not push to the limit
    return s.split('').reduce((result, value, index, array) => {
        const current = numerals[value];
        
        if (current < array[index + 1]) {
            current = -current;
        }

        return result + current;
    }, 0);
}

export function run() {
    assert_eq(roman_to_int("III"), 3);
    assert_eq(roman_to_int("LVIII"), 58);
    assert_eq(roman_to_int("MCMXCIV"), 1994);
}
