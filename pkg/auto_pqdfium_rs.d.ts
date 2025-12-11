/* tslint:disable */
/* eslint-disable */

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly malloc: (a: number) => number;
  readonly __cxa_free_exception: (a: number) => void;
  readonly _initialize: () => void;
  readonly htonl: (a: number) => number;
  readonly htons: (a: number) => number;
  readonly ntohs: (a: number) => number;
  readonly setThrew: (a: number, b: number) => void;
  readonly _emscripten_tempret_set: (a: number) => void;
  readonly _emscripten_stack_restore: (a: number) => void;
  readonly emscripten_stack_get_current: () => number;
  readonly __cxa_decrement_exception_refcount: (a: number) => void;
  readonly __cxa_increment_exception_refcount: (a: number) => void;
  readonly __cxa_can_catch: (a: number, b: number, c: number) => number;
  readonly __cxa_get_exception_ptr: (a: number) => number;
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
