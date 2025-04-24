<script lang="ts">
  import MonacoEditor from "svelte-monaco";
  import * as Monaco from "monaco-editor";
  import { getCommandsDescriptions } from "../lib/nushell";
  import { spanToRange } from "../lib/utils";
  import FilesBar from "./FilesBar.svelte";
  import Run from "~icons/si/terminal-duotone";

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

  function handleEditorReady(
    event: CustomEvent<Monaco.editor.IStandaloneCodeEditor>,
  ) {
    editor = event.detail;
    editor.addCommand(Monaco.KeyCode.Enter, sendCode);
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
  const onFileClick = (file) => {
    if (!editor) return;
    const position = editor.getPosition();
    if (!code || !position) {
      code = `cat /files/${file}`;
      return;
    }
    const offset = editor.getModel()?.getOffsetAt(position);
    if (offset === undefined) return;
    code = `${code.substring(0, offset)}/files/${file} ${code.substring(offset)}`;
  };
</script>

<div class="grid grid-rows-[auto_1fr] gap-2 mx-2">
  <FilesBar class="w-full" {onFileClick} />
  <div class="grid grid-cols-[1fr_auto] w-full gap-2 my-2 items-center">
    <div class="h-full max-h-[99%] border-r-2 border-2 border-gray-500">
      <MonacoEditor
        bind:value={code}
        on:ready={handleEditorReady}
        options={{
          language: "shell",
          theme: "vs-dark",
          minimap: { enabled: false },
          scrollbar: { vertical: "hidden", horizontal: "hidden" },
          automaticLayout: true,
        }}
      />
    </div>
    <button
      class="btn bg-green-700 transition-colors duration-200 hover:bg-green-800"
      disabled={!code}
      onclick={sendCode}>Run <Run /></button
    >
  </div>
</div>
