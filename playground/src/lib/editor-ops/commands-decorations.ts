import * as monaco from "monaco-editor";

import {
  getCommandsDescriptions,
  getNextSpanStart,
} from "../../wasm/nushell_wasm";
import { moveSpanByOffset, spanToRange } from "../utils";

export const addCommandsDecoration = (
  editor: monaco.editor.IStandaloneCodeEditor,
): monaco.IDisposable | undefined => {
  let commandsDecoration:
    | monaco.editor.IEditorDecorationsCollection
    | undefined;
  return editor.getModel()?.onDidChangeContent(() => {
    commandsDecoration?.clear();
    const code = editor.getValue();
    const commandsDescriptions = getCommandsDescriptions(code);
    commandsDecoration = editor.createDecorationsCollection(
      commandsDescriptions.map((commandDescription) => {
        const alignedSpan = moveSpanByOffset(
          commandDescription.span,
          getNextSpanStart() * -1,
        );

        return {
          range: spanToRange(code, alignedSpan),
          options: {
            hoverMessage: { value: commandDescription.description },
          },
        };
      }),
    );
  });
};
