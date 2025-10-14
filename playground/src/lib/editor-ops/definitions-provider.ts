import * as monaco from "monaco-editor";
import { LANG } from "../constants";
import {
  getDeclarationNameFromId,
  getNextSpanStart,
  getPipelineElementByOffset,
} from "../nushell";
import { moveSpanByOffset, spanToRange } from "../utils";

export const addDefinitionProvider = (
  editor: monaco.editor.IStandaloneCodeEditor,
): monaco.IDisposable => {
  const fakeUri = monaco.Uri.parse("inmemory://fake");
  const fakeModel = monaco.editor.createModel("", LANG, fakeUri);
  // copied from https://stackoverflow.com/questions/65499301/how-to-override-editor-services
  // since I had to override the built-in `openCodeEditor` function to open another tab instead
  // it was hard.
  // @ts-expect-error this is a monkey patch
  editor._codeEditorService.openCodeEditor = async (
    data: { options: { selection: monaco.Selection } },
    editor: monaco.editor.IStandaloneCodeEditor,
  ) => {
    const range = data.options.selection as monaco.Range;
    const model = editor.getModel();

    if (!model) return;

    // find pipeline element
    const offset = model.getOffsetAt(range.getStartPosition());
    const pipelineElement = getPipelineElementByOffset(
      model.getValue(),
      offset,
    );

    if (!pipelineElement) return;

    // get declaration name of pipeline element
    const name = getDeclarationNameFromId(pipelineElement.expr.Call.decl_id);

    if (!name) return;

    // open declaration docs
    const nushellDocsBaseUrl =
      import.meta.env.VITE_NUSHELL_DOCS_BASE_URL ??
      "https://www.nushell.sh/commands/docs";
    const nameInDocs = name.replaceAll(" ", "_");
    const newTab = window.open(
      `${nushellDocsBaseUrl}/${nameInDocs}.html`,
      "_blank",
    );
    // Using the focus method does not really works on modern browsers since this is deprecated for better UX.
    // JS code cannot change tabs for the user, only user actions can (like clicking an <a> element)
    newTab?.focus();
  };

  const defintionProvider = monaco.languages.registerDefinitionProvider(LANG, {
    provideDefinition(model, position) {
      // find pipeline element
      const offset = model.getOffsetAt(position);
      const code = model.getValue();
      const pipelineElement = getPipelineElementByOffset(code, offset);

      if (!pipelineElement) return;

      // We need to align the span since the engine state accumulate spans and each time the next spans starts from the end of the previous spans.
      // without the alignment this feature will only work when Ctrl+clicking on elements in the first command.
      const pipelineElementHeadSpan = pipelineElement.expr.Call.head;
      const pipelineElementHeadalignedSpan = moveSpanByOffset(
        pipelineElementHeadSpan,
        getNextSpanStart() * -1,
      );

      const pipelineElementHeadalignedRange = spanToRange(
        code,
        pipelineElementHeadalignedSpan,
      );
      if (
        // The nushell function returns the entire call expression also if an argument was clicked.
        // so we check that the clicked position is the head of the call expression (the actual command)
        !pipelineElementHeadalignedRange.containsPosition(position)
      ) {
        return null;
      }

      return [
        {
          uri: fakeUri,
          // pass the range in the modal for the `openCodeEditor` method monkey-patch
          // to get later then open in nushell docs.
          range: pipelineElementHeadalignedRange,
        },
      ];
    },
  });

  return {
    dispose() {
      fakeModel.dispose();
      defintionProvider.dispose();
    },
  };
};
