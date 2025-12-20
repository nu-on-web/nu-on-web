import type { RunCodeResult } from "../wasm/nushell_wasm";

export type Message = (
  | { type: "user"; result: string }
  | { type: "nushell"; result: RunCodeResult }
) & { time: Date };
