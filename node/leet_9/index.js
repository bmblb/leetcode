var isPalindrome = function(x) {
    return x.toString() === x.toString().split('').reverse().join('');
};

if (!isPalindrome(121)) {
    console.log('fail');
}
if (isPalindrome(-121)) {
    console.log('fail');
}
if (isPalindrome(10)) {
    console.log('fail');
}
if (!isPalindrome(12233221)) {
    console.log('fail');
}
