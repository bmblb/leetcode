import { assert_eq } from "../utils.mjs";

function longestCommonPrefix(strs) {
    // Return early on empty input
    if (!strs.length) return '';

    // Loop through the letters of the first word
    for (let i = 0; i <= strs[0].length; i++) {
        // Check if this character is present in the same position of every string
        if (!strs.every((string) => string[i] === strs[0][i])) {
            // If not, return the string up to and including the previous character
            return strs[0].slice(0, i);
        }
    }

    return strs[0];
};

export function run() {
    assert_eq(longestCommonPrefix(["flower", "flow", "flight"]), 'fl');
    assert_eq(longestCommonPrefix(["dog","racecar","car"]), '');
}