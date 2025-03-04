import { z } from 'zod'

// const Types = z.enum(['Int', 'Nothing', 'String', 'List'])
// const BaseResult = z.discriminatedUnion('type', [
//   z.object({
//     type: z.literal(Types.enum.Nothing),
//   }),
//   z.object({
//     type: z.literal(Types.enum.Int),
//     val: z.number().int()
//   }),
//   z.object({
//     type: z.literal(Types.enum.String),
//     val: z.string()
//   }),
// ])
// 
// 
// type _Result = z.infer<typeof BaseResult> | {
//   type: "List",
//   vals: _Result[]
// }
// 
// export const Result: z.ZodType<_Result> = z.discriminatedUnion('type', [
//   ...BaseResult.options,
//   z.object({ type: z.literal(Types.enum.List), vals: z.lazy(() => Result.array()) })
// ])
// 
// export type Result = z.infer<typeof Result>


export const BaseResult = z.object({ Nothing: z.object({}) }).or(z.object({
  Int: z.object({ val: z.number().int() })
})).or(z.object({
  Float: z.object({ val: z.number() })
})).or(z.object({
  String: z.object({ val: z.string() })
}))

type _Result = z.infer<typeof BaseResult> | {
  List: {
    vals: _Result[]
  }
}

export const Result: z.ZodType<_Result> = BaseResult.or(z.object({
  List: z.object({ vals: z.lazy(() => Result.array()) })
}))

export type Result = z.infer<typeof Result>
