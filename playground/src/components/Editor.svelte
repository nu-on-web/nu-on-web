<script lang="ts">
  import * as monaco from "monaco-editor";
  import { LANG } from "../lib/constants";
  import {
    addDefinitionProvider,
    addCommandsDecoration,
    addAutoCompletions,
  } from "../lib/editor-ops/";
  import type { Attachment } from "svelte/attachments";
  import { untrack } from "svelte";
  import { history } from "../lib/stores/code";
  import { code as codeGlobal } from "../lib/stores/code";

  interface Props {
    code?: string;
    disable?: boolean;
    onEnter?: () => void;
  }

  const IS_AUTOCOMPLETE_OPEN_CTX_KEY = "isAutoCompleteOpen";

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
    const model = editor?.getModel();
    if (!model || !editor) return;

    const selection = editor.getSelection();
    if (!code || !selection) {
      editor.setValue(text);
      const newPos = model.getPositionAt(text.length);
      editor.setPosition(newPos);
      return;
    }

    const startOffset = model.getOffsetAt(selection.getStartPosition());
    const endOffset = model.getOffsetAt(selection.getEndPosition());
    editor.setValue(
      `${code.substring(0, startOffset)}${text}${code.substring(endOffset)}`,
    );
    const newPos = model.getPositionAt(startOffset + text.length);
    editor.setPosition(newPos);
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

    let isAutoCompleteOpenCtx = editor.createContextKey<boolean>(
      IS_AUTOCOMPLETE_OPEN_CTX_KEY,
      false,
    );

    const suggestController = editor.getContribution(
      "editor.contrib.suggestController",
    ) as any;

    suggestController?.widget.value.onDidShow(() =>
      isAutoCompleteOpenCtx.set(true),
    );

    suggestController?.widget.value.onDidHide(() =>
      isAutoCompleteOpenCtx.set(false),
    );

    editor.addCommand(
      monaco.KeyCode.Enter,
      () => onEnter?.(),
      `!${IS_AUTOCOMPLETE_OPEN_CTX_KEY}`, // run onEnter only when the suggestion popup is closed
    );

    editor.onKeyDown((e) => {
      if (!editor) return;

      if (e.keyCode === monaco.KeyCode.UpArrow) {
        console.log($history);
        const position = editor.getPosition();
        // At the top of the document
        if (position && position.lineNumber === 1) {
          e.preventDefault();
          $codeGlobal =
            $history.up(editor.getValue().trim()) ?? editor.getValue().trim();
        }
      }

      if (e.keyCode === monaco.KeyCode.DownArrow) {
        const position = editor.getPosition();
        const model = editor.getModel();
        // At the bottom of the document
        if (position && model && position.lineNumber === model.getLineCount()) {
          e.preventDefault();
          $codeGlobal = $history.down() ?? editor.getValue().trim();
        }
      }
    });

    const autoCompleter = addAutoCompletions();

    return () => {
      changeContentListener?.dispose();
      definitionProvider.dispose();
      commandsDecoration?.dispose();
      autoCompleter.dispose();
      editor?.dispose();
    };
  };
</script>

<div class="h-full" {@attach editorAttachment}></div>
