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

<div class="h-full flex flex-col gap-4">
  <FilesBar class="w-full" {onFileClick} />
  <div class="flex flex-1 w-full gap-4 items-stretch bg-gray-800 p-4 rounded-lg overflow-hidden">
    <div class="flex-1 border border-gray-600 rounded-md overflow-hidden">
      <MonacoEditor
        bind:value={code}
        on:ready={handleEditorReady}
        options={{
          language: "shell",
          theme: "vs-dark",
          minimap: { enabled: false },
          scrollbar: { vertical: "auto", horizontal: "auto" },
          automaticLayout: true,
          lineNumbers: "off",
          wordWrap: "on",
          fontSize: 14,
        }}
      />
    </div>
    <button
      class="btn btn-success px-4 py-2 self-start flex items-center gap-2"
      disabled={!code}
      on:click={sendCode}
    >
      <Run class="w-5 h-5" /> Run
    </button>
  </div>
</div>
