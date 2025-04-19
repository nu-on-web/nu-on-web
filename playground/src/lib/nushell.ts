import init, {
  run_code,
  get_commands_descriptions,
  find_pipeline_element_by_position,
  get_declaration,
  get_next_span_start,
} from "../wasm/nushell_wasm";
import { z } from 'zod'

import { Result, GetCommandsDescriptionsResult, PipelineElement } from "./types";

export async function runCode(code: string): Promise<Result> {
  await init();
  return Result.parse(run_code(code));
}

export async function getCommandsDescriptions(
  code: string,
): Promise<GetCommandsDescriptionsResult> {
  await init();
  return GetCommandsDescriptionsResult.parse(get_commands_descriptions(code));
}

export function getPipelineElementByOffset(code: string, offset: number) {
  const result = find_pipeline_element_by_position(code, offset)
  return PipelineElement.optional().parse(result)
}

export function getDeclarationNameFromId(declarationId: number) {
  return z.string().optional().parse(get_declaration(declarationId))
}
export function getNextSpanStart() {
  return z.number().parse(get_next_span_start())
}
