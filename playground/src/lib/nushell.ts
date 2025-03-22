import init, {
  run_code,
  get_commands_descriptions,
} from "../wasm/nushell_wasm";

import { Result, GetCommandsDescriptionsResult } from "./types";

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
