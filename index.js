const argon2 = require('.');

// 12345
// $argon2id$v=19$m=19456,t=2,p=1$7I5nRoN86YCQw8anJnJozQ$uGgujBNS6knu0a6rZZGgybLZXHVn3b8jLcKwvCnX8GI

const plaintext = "12345";

console.log("Hashing...");
const hash = argon2.hash(plaintext);
console.log(hash);

let isVerified;

console.log("Verifying...");
isVerified = argon2.verify(plaintext, hash)
console.log(isVerified ? "Verified (should match)" : "!!! NOT VERIFIED !!! (SHOULD MATCH)");

isVerified = argon2.verify(plaintext + 'x', hash)
console.log(!isVerified ? "Verified (should not match)" : "!!! NOT VERIFIED !!! (SHOULD NOT MATCH)");
