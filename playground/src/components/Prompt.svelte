<script lang="ts">
  import * as monaco from "monaco-editor";
  import Editor from "./Editor.svelte";
  import FilesBar from "./FilesBar.svelte";
  import Run from "~icons/si/terminal-duotone";
  import { isEmpty } from "lodash-es";

  interface Props {
    onSend: (code: string) => void;
    disable?: boolean;
  }

  const { onSend, disable }: Props = $props();

  let code = $state("");
  let editor: Editor;

  function sendCode() {
    if (disable) return;
    onSend(code);
  }

  const onFileClick = (file: string) => {
    if (!editor) return;
    const text = isEmpty(code) ? `cat /files/${file}` : `/files/${file} `;
    editor.insertAtCursor(text);
    editor.focus();
  };
</script>

<div class="h-full flex flex-col gap-4">
  <FilesBar class="w-full" {onFileClick} />
  <div
    class="flex flex-1 w-full gap-4 items-stretch bg-gray-800 p-4 rounded-lg"
  >
    <div class="flex-1 border border-gray-600 rounded-md">
      <Editor bind:this={editor} bind:code {disable} onEnter={sendCode} />
    </div>
    <button
      class="btn btn-success px-4 py-2 self-start flex items-center gap-2"
      disabled={!code}
      onclick={sendCode}
    >
      <Run class="w-5 h-5" /> Run
    </button>
  </div>
</div>
