<script lang="ts">
  import MonacoEditor from "svelte-monaco";
  import * as Monaco from "monaco-editor";
  import { getCommandsDescriptions } from "../lib/nushell";
  import { spanToRange } from "../lib/utils";

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
</script>

<div class="flex gap-2 m-2 h-15 items-center">
  <div class="grow w-full h-full border-r-2 border-2 border-gray-500">
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
  <button class="btn btn-primary" onclick={sendCode}>send</button>
</div>
