<script lang="ts">
  import * as monaco from "monaco-editor";
  import { LANG } from "../lib/constants";
  import {
    addDefinitionProvider,
    addCommandsDecoration,
    addAutoCompletions,
  } from "../lib/editor-ops/";

  interface Props {
    code?: string;
    editor?: monaco.editor.IStandaloneCodeEditor;
    disable?: boolean;
    onEnter?: () => void;
  }
  let {
    code = $bindable(),
    editor = $bindable(),
    disable,
    onEnter,
  }: Props = $props();

  let editorContainer = $state<HTMLDivElement>();

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
      tabCompletion: "on",
      fontSize: 14,
      padding: { top: 14 },
    });
    return () => editor?.dispose();
  });

  $effect(() => {
    if (!editor) return;
    const definitionProvider = addDefinitionProvider(editor);

    const commandsDecoration = addCommandsDecoration(editor);

    const autoCompleter = addAutoCompletions();

    return () => {
      definitionProvider.dispose();
      commandsDecoration?.dispose();
      autoCompleter.dispose();
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
    const controller = editor.getContribution(
      "editor.contrib.suggestController",
    ) as any;
    editor.addCommand(
      monaco.KeyCode.Enter,
      () => {
        console.log(123, controller);
        if (controller?.model?.state) return;
        onEnter?.();
      },
      "!autoCompleteIsOpen",
    );
  });
  $effect(() => {
    if (!editor) return;
    let ctx = editor.createContextKey<boolean>("autoCompleteIsOpen", false);
    const suggestController = editor.getContribution(
      "editor.contrib.suggestController",
    );

    // Check if the suggest controller and widget exist
    if (suggestController && suggestController.widget) {
      // Listen for the suggestion popup opening
      suggestController.widget.value.onDidShow(() => {
        ctx.set(true);
      });

      // Listen for the suggestion popup closing
      suggestController.widget.value.onDidHide(() => {
        ctx.set(false);
      });
    }
  });
</script>

<div class="h-full" bind:this={editorContainer}></div>
