<script lang="ts">
  import * as Monaco from "monaco-editor";
  import {
    getCommandsDescriptions,
    getDeclarationNameFromId,
    getNextSpanStart,
    getPipelineElementByOffset,
  } from "../lib/nushell";
  import { moveSpanByOffset, spanToRange } from "../lib/utils";
  import FilesBar from "./FilesBar.svelte";
  import Run from "~icons/si/terminal-duotone";
  const LANG = "shell";
  let editorContainer;

  interface Props {
    onSend: (code: string) => void;
    disable?: boolean;
  }

  const { onSend, disable }: Props = $props();

  let code = $state("");

  let editor: Monaco.editor.IStandaloneCodeEditor | undefined;
  function sendCode() {
    if (disable) return;
    onSend(code);
  }

  let commandsDecoration:
    | Monaco.editor.IEditorDecorationsCollection
    | undefined;

  $effect(() => {
    commandsDecoration?.clear();
    let active = true;
    getCommandsDescriptions(code).then((commandsDescriptions) => {
      if (!active) return;
      commandsDecoration = editor?.createDecorationsCollection(
        commandsDescriptions.map((commandDescription) => ({
          range: spanToRange(code, commandDescription.span),
          options: {
            hoverMessage: { value: commandDescription.description },
          },
        })),
      );
    });
    return () => {
      active = false;
    };
  });

  $effect(() => {
    editor?.updateOptions({ readOnly: disable ?? false });
  });

  $effect(() => {
    const fakeUri = Monaco.Uri.parse("inmemory://fake");

    const fakeModel = Monaco.editor.createModel("", LANG, fakeUri);

    editor = Monaco.editor.create(editorContainer!, {
      value: "",
      language: LANG,
      theme: "vs-dark",
      minimap: { enabled: false },
      scrollbar: { vertical: "hidden", horizontal: "hidden" },
      automaticLayout: true,
    });

    // @ts-expect-error there is no better way of doing it without fixing monaco editor code.
    editor._codeEditorService.openCodeEditor = async (
      data: any,
      editor: Monaco.editor.IStandaloneCodeEditor,
    ) => {
      const range = data.options.selection as Monaco.Range;
      const model = editor.getModel();
      if (!model) return;
      const offset = model.getOffsetAt(range.getStartPosition());
      const pipelineElement = getPipelineElementByOffset(
        model.getValue(),
        offset,
      );
      if (!pipelineElement) return;
      const name = getDeclarationNameFromId(pipelineElement.expr.Call.decl_id);
      if (!name) return;
      const nameInDocs = name.replaceAll(" ", "_");
      const newTab = window.open(
        `https://www.nushell.sh/commands/docs/${nameInDocs}.html`,
        "_blank",
      );
      // Using the focus method does not really works on modern browsers since this is deprecated for better UX.
      // js code cannot change tabs for the user, only user actions can (like clicking an <a> element)
      newTab?.focus();
    };

    const defintionProvider = Monaco.languages.registerDefinitionProvider(
      "shell",
      {
        provideDefinition(model, position, _token) {
          const offset = model.getOffsetAt(position);
          const pipelineElement = getPipelineElementByOffset(code, offset);

          if (!pipelineElement) return;
          // We need to align the span since the engine state accumulate spans and each time the next spans starts from the end of the previous spans.
          // without the alignment this feature will only work when ctrl+clicking on elements in the first command.
          const alignedSpan = moveSpanByOffset(
            pipelineElement.expr.Call.head,
            getNextSpanStart() * -1,
          );
          const range = spanToRange(code, alignedSpan);
          if (
            // The nushell function returns the entire call expression also if an argument was clicked.
            // so we check that the clicked position is the head of the call expression (the actual command)
            !range.containsPosition(position)
          ) {
            return null;
          }
          return [
            {
              uri: fakeUri,
              // pass the range in the modal for the `openCodeEditor` method monkey-patch
              // to get later then open in nushell docs.
              range: range,
            },
          ];
        },
      },
    );

    editor.getModel()?.onDidChangeContent(() => {
      code = editor?.getValue()!;
    });
    editor.addCommand(Monaco.KeyCode.Enter, sendCode);
    return () => {
      fakeModel.dispose();
      editor?.dispose();
      defintionProvider.dispose();
    };
  });

  // Copied from https://github.com/tristan-f-r/svelte-monaco/blob/f65db445b45fc294cf8e2888df112c08359820ee/src/lib/Monaco.svelte#L75C1-L81C5
  $effect(() => {
    if (editor && editor.getValue() != code) {
      const position = editor.getPosition();
      editor.setValue(code);
      if (position) editor.setPosition(position);
    }
  });
</script>

<div class="flex flex-wrap gap-2 m-2 items-center">
  <FilesBar
    class="w-full"
    onFileClick={(file) => {
      if (!editor) return;
      const position = editor.getPosition();
      if (!code || !position) {
        code = `cat /files/${file}`;
        return;
      }
      const offset = editor.getModel()?.getOffsetAt(position);
      if (offset === undefined) return;
      code = `${code.substring(0, offset)}/files/${file} ${code.substring(offset)}`;
    }}
  />
  <div class="flex w-full gap-2 mx-2 h-15 items-center">
    <div class="grow w-full h-full border-r-2 border-2 border-gray-500">
      <div bind:this={editorContainer} class="h-full"></div>
    </div>
    <button
      class="btn bg-green-700 transition-colors duration-200 hover:bg-green-800"
      disabled={!code}
      onclick={sendCode}>Run <Run /></button
    >
  </div>
</div>
