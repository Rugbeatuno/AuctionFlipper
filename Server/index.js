const ffi = require('ffi-napi');
const lib = ffi.Library('../target/x86_64-apple-darwin/release/libauctionhouse', {
    'flipper': ['int', []]
});

let _ = lib.flipper();
