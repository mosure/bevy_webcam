/* tslint:disable */
/* eslint-disable */

export function frame_input(pixel_data: Uint8Array, width: number, height: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly frame_input: (a: number, b: number, c: number, d: number) => void;
    readonly main: (a: number, b: number) => number;
    readonly wasm_bindgen_673083768d9b494a___closure__destroy___dyn_core_d7ff8db7104fb838___ops__function__FnMut__wasm_bindgen_673083768d9b494a___JsValue____Output___core_d7ff8db7104fb838___result__Result_____wasm_bindgen_673083768d9b494a___JsError___: (a: number, b: number) => void;
    readonly wasm_bindgen_673083768d9b494a___closure__destroy___dyn_core_d7ff8db7104fb838___ops__function__FnMut__wasm_bindgen_673083768d9b494a___JsValue____Output_______: (a: number, b: number) => void;
    readonly wasm_bindgen_673083768d9b494a___closure__destroy___dyn_core_d7ff8db7104fb838___ops__function__FnMut__wasm_bindgen_673083768d9b494a___JsValue____Output________1_: (a: number, b: number) => void;
    readonly wasm_bindgen_673083768d9b494a___convert__closures_____invoke___wasm_bindgen_673083768d9b494a___JsValue__core_d7ff8db7104fb838___result__Result_____wasm_bindgen_673083768d9b494a___JsError___1_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_673083768d9b494a___convert__closures_____invoke___js_sys_b62b17bb973c40a0___Array__web_sys_25e3c889708ac7cc___features__gen_ResizeObserver__ResizeObserver_____: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_673083768d9b494a___convert__closures_____invoke___wasm_bindgen_673083768d9b494a___JsValue_____: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_673083768d9b494a___convert__closures_____invoke___wasm_bindgen_673083768d9b494a___JsValue______1_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_673083768d9b494a___convert__closures_____invoke___wasm_bindgen_673083768d9b494a___JsValue______1__4: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_673083768d9b494a___convert__closures_____invoke___wasm_bindgen_673083768d9b494a___JsValue______1__5: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_673083768d9b494a___convert__closures_____invoke___wasm_bindgen_673083768d9b494a___JsValue______1__6: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_673083768d9b494a___convert__closures_____invoke___wasm_bindgen_673083768d9b494a___JsValue______1__7: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_673083768d9b494a___convert__closures_____invoke___wasm_bindgen_673083768d9b494a___JsValue______1__8: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_673083768d9b494a___convert__closures_____invoke___wasm_bindgen_673083768d9b494a___JsValue______1__9: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_673083768d9b494a___convert__closures_____invoke___wasm_bindgen_673083768d9b494a___JsValue______1__10: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_673083768d9b494a___convert__closures_____invoke______: (a: number, b: number) => void;
    readonly __wbindgen_malloc_command_export: (a: number, b: number) => number;
    readonly __wbindgen_realloc_command_export: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc_command_export: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store_command_export: (a: number) => void;
    readonly __wbindgen_free_command_export: (a: number, b: number, c: number) => void;
    readonly __externref_table_dealloc_command_export: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
