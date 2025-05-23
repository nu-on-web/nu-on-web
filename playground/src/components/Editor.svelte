<script lang="ts">
  import * as monaco from "monaco-editor";
  import { LANG } from "../lib/constants";
  import {
    addDefinitionProvider,
    addCommandsDecoration,
  } from "../lib/editor-ops";

  interface Props {
    code?: string;
    disable?: boolean;
    onEnter?: () => void;
  }

  let { code = $bindable(), disable, onEnter }: Props = $props();

  let editorContainer = $state<HTMLDivElement>();
  let editor: monaco.editor.IStandaloneCodeEditor | undefined;

  $effect(() => {
    if (!editorContainer) return;
    editor = monaco.editor.create(editorContainer, {
      language: LANG,
      theme: "vs-dark",
      minimap: { enabled: false },
      scrollbar: { vertical: "auto", horizontal: "auto" },
      automaticLayout: true,
      lineNumbers: "off",
      wordWrap: "on",
      fontSize: 14,
      padding: { top: 14 },
    });
    return () => editor?.dispose();
  });

  $effect(() => {
    if (!editor) return;
    const definitionProvider = addDefinitionProvider(editor);

    const commandsDecoration = addCommandsDecoration(editor);

    return () => {
      definitionProvider.dispose();
      commandsDecoration?.dispose();
    };
  });

  $effect(() => {
    editor?.updateOptions({ readOnly: disable ?? false });
  });

  // Copied from https://github.com/tristan-f-r/svelte-monaco/blob/f65db445b45fc294cf8e2888df112c08359820ee/src/lib/Monaco.svelte#L75C1-L81C5
  $effect(() => {
    if (editor && code && editor.getValue() !== code) {
      const position = editor.getPosition();
      editor.setValue(code);
      if (position) editor.setPosition(position);
    }
  });

  $effect(() => {
    const changeContentListener = editor?.getModel()?.onDidChangeContent(() => {
      if (!editor) return;
      code = editor.getValue();
    });
    return () => changeContentListener?.dispose();
  });

  $effect(() => {
    if (!editor) return;
    editor.addCommand(monaco.KeyCode.Enter, () => onEnter?.());
  });

  export function focus() {
    editor?.focus();
  }

  export function insertAtCursor(text: string) {
    if (!editor) return;
    const position = editor.getPosition();
    if (!code || !position) {
      code = text;
      return;
    }
    const offset = editor.getModel()?.getOffsetAt(position);
    if (offset === undefined) return;
    code = `${code.substring(0, offset)}${text}${code.substring(offset)}`;
  }
</script>

<div class="h-full" bind:this={editorContainer}></div>
