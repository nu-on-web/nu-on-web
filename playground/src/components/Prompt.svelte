<script lang="ts">
  import * as monaco from "monaco-editor";
  import Editor from "./Editor.svelte";
  import FilesBar from "./FilesBar.svelte";
  import Run from "~icons/si/terminal-duotone";

  interface Props {
    onSend: (code: string) => void;
    disable?: boolean;
  }

  const { onSend, disable }: Props = $props();

  let code = $state("");
  let editor = $state<monaco.editor.IStandaloneCodeEditor>();

  function sendCode() {
    if (disable) return;
    onSend(code);
  }

  const onFileClick = (file: string) => {
    if (!editor) return;
    const position = editor.getPosition();
    const insertPath = `/files/${file}`;
    if (!code || !position) {
      code = `cat ${insertPath}`;
      editor.focus();
      return;
    }
    const offset = editor.getModel()?.getOffsetAt(position);
    if (offset === undefined) {
      editor.focus();
      return;
    }
    code = `${code.substring(0, offset)}${insertPath} ${code.substring(offset)}`;
    editor.focus();
  };
</script>

<div class="h-full flex flex-col gap-4">
  <FilesBar class="w-full" {onFileClick} />
  <div
    class="flex flex-1 w-full gap-4 items-stretch bg-gray-800 p-4 rounded-lg overflow-hidden"
  >
    <div class="flex-1 border border-gray-600 rounded-md overflow-hidden">
      <Editor bind:editor bind:code {disable} onEnter={sendCode} />
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
