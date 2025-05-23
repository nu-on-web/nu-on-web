import { z } from "zod";

export const Span = z.object({
  start: z.number(),
  end: z.number(),
});
export type Span = z.infer<typeof Span>;

const CompileError = z.object({
  RunExternalNotFound: z.object({ span: Span }),
});

export const Result = z
  .object({
    success: z.object({ String: z.object({ val: z.string() }) }),
  })
  .or(
    z.object({
      compileErrors: z.array(CompileError),
    }),
  )
  .or(
    z.object({
      parseErrors: z.array(z.object({})),
    }),
  );
export type Result = z.infer<typeof Result>;

export const GetCommandsDescriptionsResult = z.array(
  z.object({
    description: z.string(),
    span: Span,
  }),
);
export type GetCommandsDescriptionsResult = z.infer<
  typeof GetCommandsDescriptionsResult
>;

export const Message = z
  .discriminatedUnion("type", [
    z.object({
      type: z.literal("user"),
      value: z.string(),
    }),
    z.object({
      type: z.literal("nushell"),
      value: Result,
    }),
  ])
  .and(
    z.object({
      time: z.date(),
    }),
  );
export type Message = z.infer<typeof Message>;

export const Expression = z.object({
  Call: z.object({
    head: Span,
    decl_id: z.number()
  })
})
export type Expression = z.infer<typeof Expression>

export const PipelineElement = z.object({
  expr: Expression,
  span: Span
})
export type PipelineElement = z.infer<typeof PipelineElement>
