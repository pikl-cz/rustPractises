/* tslint:disable */
/* eslint-disable */

export class Body {
  free(): void;
  [Symbol.dispose](): void;
  constructor(x: number, y: number);
  step(dt: number): void;
  x: number;
  y: number;
  vx: number;
  vy: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_body_free: (a: number, b: number) => void;
  readonly __wbg_get_body_vx: (a: number) => number;
  readonly __wbg_get_body_vy: (a: number) => number;
  readonly __wbg_get_body_x: (a: number) => number;
  readonly __wbg_get_body_y: (a: number) => number;
  readonly __wbg_set_body_vx: (a: number, b: number) => void;
  readonly __wbg_set_body_vy: (a: number, b: number) => void;
  readonly __wbg_set_body_x: (a: number, b: number) => void;
  readonly __wbg_set_body_y: (a: number, b: number) => void;
  readonly body_new: (a: number, b: number) => number;
  readonly body_step: (a: number, b: number) => void;
  readonly __wbindgen_externrefs: WebAssembly.Table;
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
