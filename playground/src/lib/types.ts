import { z } from 'zod'

export const Span = z.object({
  start: z.number(),
  end: z.number(),
})
export type Span = z.infer<typeof Span>

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

export const GetCommandsDescriptionsResult = z.array(z.object({
  description: z.string(),
  span: Span
}))
export type GetCommandsDescriptionsResult = z.infer<typeof GetCommandsDescriptionsResult>;
