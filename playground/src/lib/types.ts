import type { RunCodeResult } from "../wasm/nushell_wasm";

export type Message = (
  | { type: "user"; value: string }
  | { type: "nushell"; value: RunCodeResult }
) & { time: Date };

// export const Message = z
//   .discriminatedUnion("type", [
//     z.object({
//       type: z.literal("user"),
//       value: z.string(),
//     }),
//     z.object({
//       type: z.literal("nushell"),
//       value: z.custom<RunCodeResult>((t) => t),
//     }),
//   ])
//   .and(
//     z.object({
//       time: z.date(),
//     }),
//   );
// export type Message = z.infer<typeof Message>;
