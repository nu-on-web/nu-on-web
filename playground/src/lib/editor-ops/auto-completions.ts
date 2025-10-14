import * as monaco from "monaco-editor";
import { LANG } from "../constants";
import { fetchCompletions, getNextSpanStart } from "../nushell";

export const addAutoCompletions = (): monaco.IDisposable => {
  const autoCompleter = monaco.languages.registerCompletionItemProvider(LANG, {
    provideCompletionItems(model, position) {
      const offset = model.getOffsetAt(position);
      const code = model.getValue();
      const [span, suggestedCommands] = fetchCompletions(
        code,
        offset - (code.length === offset ? 1 : 0),
      );
      if (!span) return { suggestions: [] };

      return {
        suggestions: suggestedCommands.map((cmd) => {
          const nextSpanStart = getNextSpanStart();
          const start = model.getPositionAt(span.start - nextSpanStart);
          const end = model.getPositionAt(span.end - nextSpanStart);

          return {
            label: cmd,
            kind: monaco.languages.CompletionItemKind.Method,
            insertText: cmd,
            range: monaco.Range.fromPositions(start, end),
          };
        }),
      };
    },
  });

  return autoCompleter;
};
