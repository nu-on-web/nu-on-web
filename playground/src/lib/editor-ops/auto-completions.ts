import * as monaco from 'monaco-editor';
import { LANG } from '../constants';
import { fetchCompletions } from '../nushell';

export const addAutoCompletions = (): monaco.IDisposable => {

  const autoCompleter = monaco.languages.registerCompletionItemProvider(LANG, {
    provideCompletionItems(model, position, a, b) {
      const offset = model.getOffsetAt(position)
      const code = model.getValue();
      console.log(111111111111, code.length, offset)
      const suggestedCommands = fetchCompletions(code, offset - (code.length === offset ? 1 : 0))
      console.log({ offset, code, suggestedCommands })

      return {
        suggestions: suggestedCommands.map(cmd => {
          const start = model.getPositionAt(offset - 1)
          const end = model.getPositionAt(offset + cmd.length)
          return ({
            label: cmd,
            kind: monaco.languages.CompletionItemKind.Method,
            insertText: cmd,
            range: monaco.Range.fromPositions(start, end)
          })
        })
      }
    }
  })

  return autoCompleter
}

