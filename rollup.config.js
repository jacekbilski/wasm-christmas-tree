import rust from "@wasm-tool/rollup-plugin-rust";

export default {
    input: {
        feature: "./feature/stage1.js"
    },
    output: {
        dir: 'build/stage1',
        format: 'es',
    },
    plugins: [
        rust({inlineWasm: true}),
    ],
};
