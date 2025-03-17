import { __wbg_set_wasm } from "./pkg/internal_bg.js";
import * as wasm from "./pkg/internal_bg.wasm";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();

export { memory } from './pkg/internal_bg.wasm';
export * from "./pkg/internal_bg.js";
