import init, {
  run_code,
  get_commands_descriptions,
  find_pipeline_element_by_position,
  get_declaration,
  get_next_span_start,
  fetch_completions,
} from "../wasm/nushell_wasm";
import { z } from "zod";

import {
  Result,
  GetCommandsDescriptionsResult,
  PipelineElement,
  Span,
} from "./types";

export { init };

export function runCode(code: string): Result {
  return Result.parse(run_code(code));
}

export function getCommandsDescriptions(
  code: string,
): GetCommandsDescriptionsResult {
  return GetCommandsDescriptionsResult.parse(get_commands_descriptions(code));
}

export function getPipelineElementByOffset(code: string, offset: number) {
  const result = find_pipeline_element_by_position(code, offset);
  return PipelineElement.optional().parse(result);
}

export function getDeclarationNameFromId(declarationId: number) {
  return z.string().optional().parse(get_declaration(declarationId));
}
export function getNextSpanStart() {
  return z.number().parse(get_next_span_start());
}

export function fetchCompletions(code: string, pos: number) {
  return z
    .tuple([Span.optional(), z.string().array()])
    .parse(fetch_completions(code, pos));
}
