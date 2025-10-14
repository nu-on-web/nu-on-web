import type { RunCodeResult } from "../wasm/nushell_wasm";

export type Message = (
  | { type: "user"; value: string }
  | { type: "nushell"; value: RunCodeResult }
) & { time: Date };
