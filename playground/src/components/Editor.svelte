<script lang="ts">
  import * as monaco from "monaco-editor";
  import { LANG } from "../lib/constants";
  import {
    addDefinitionProvider,
    addCommandsDecoration,
  } from "../lib/editor-ops";
  import type { Attachment } from "svelte/attachments";
  import { untrack } from "svelte";

  interface Props {
    code?: string;
    disable?: boolean;
    onEnter?: () => void;
  }

  let { code = $bindable(), disable, onEnter }: Props = $props();

  let editor: monaco.editor.IStandaloneCodeEditor | undefined;

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

  export function focus() {
    editor?.focus();
  }

  export function insertAtCursor(text: string) {
    if (!editor) return;
    const selection = editor.getSelection();
    if (!code || !selection) {
      editor.setValue(text);
      editor.setPosition({ lineNumber: 1, column: text.length + 1 });
      return;
    }

    const startOffset = editor
      .getModel()
      ?.getOffsetAt(selection.getStartPosition());
    const endOffset = editor
      .getModel()
      ?.getOffsetAt(selection.getEndPosition());
    if (startOffset === undefined || endOffset === undefined) return;
    editor.setValue(
      `${code.substring(0, startOffset)}${text}${code.substring(endOffset)}`,
    );
    editor.setPosition(selection.getStartPosition().delta(0, text.length));
  }

  const editorAttachment: Attachment<HTMLDivElement> = (element) => {
    editor = monaco.editor.create(element, {
      language: LANG,
      theme: "vs-dark",
      minimap: { enabled: false },
      scrollbar: { vertical: "auto", horizontal: "auto" },
      automaticLayout: true,
      lineNumbers: "off",
      wordWrap: "on",
      fontSize: 14,
      padding: { top: 14 },
      value: untrack(() => code),
      readOnly: untrack(() => disable ?? false),
    });

    const definitionProvider = addDefinitionProvider(editor);
    const commandsDecoration = addCommandsDecoration(editor);

    const changeContentListener = editor.getModel()?.onDidChangeContent(() => {
      code = editor!.getValue();
    });

    editor.addCommand(monaco.KeyCode.Enter, () => onEnter?.());

    return () => {
      changeContentListener?.dispose();
      definitionProvider.dispose();
      commandsDecoration?.dispose();
      editor?.dispose();
    };
  };
</script>

<div class="h-full" {@attach editorAttachment}></div>
