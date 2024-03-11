const assert = require('assert');

const argon2 = require('.');

// 12345
// $argon2id$v=19$m=19456,t=2,p=1$7I5nRoN86YCQw8anJnJozQ$uGgujBNS6knu0a6rZZGgybLZXHVn3b8jLcKwvCnX8GI

const plaintext = "12345";

const hash = argon2.hash(plaintext);

assert(argon2.verify(plaintext, hash));
assert(!argon2.verify(plaintext + 'x', hash));

console.log('OK');
