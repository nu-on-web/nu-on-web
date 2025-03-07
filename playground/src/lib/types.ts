import { z } from 'zod'

const Span = z.object({
  start: z.number(),
  end: z.number(),
})

const CompileError = z.object({
  RunExternalNotFound: z.object({ span: Span })
})

export const Result = z.object({
  success: z.object({ String: z.object({ val: z.string() }) })
}).or(z.object({
  compileErrors: z.array(CompileError)
})).or(z.object({
  parseErrors: z.array(z.object({}))
}))
