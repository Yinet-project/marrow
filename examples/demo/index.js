const runtime = require('tools');

// modules = {
//     keystore: {
//         path: 'target/wasm32-unkonwn-unknown-release/release/keystore.wasm',
//         deps:[],
//     }
// }

const modules = [
    {
        name: "demo",
        path: "target/wasm32-unknown-unknown/release/demo.wasm"
    }
];

runtime.test(modules);
