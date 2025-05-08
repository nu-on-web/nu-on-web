<script lang="ts">
  import Prompt from "./components/Prompt.svelte";
  import Chat from "./components/Chat.svelte";
  import { runCode } from "./lib/nushell";
  import type { Message } from "./lib/types";
  import Split from "split-grid";

  let messages = $state<Message[]>([]);
  let proccessing = $state(false);

  function onSend(code: string) {
    messages = [...messages, { type: "user", value: code, time: new Date() }];
    proccessing = true;
    (async () => {
      const codeToRun = `${code} | to html -d --partial`;
      const result = await runCode(codeToRun);
      messages = [
        ...messages,
        { type: "nushell", value: result, time: new Date() },
      ];
      proccessing = false;
    })();
  }
  let gutter = $state<HTMLDivElement>();
  $effect(() => {
    if (!gutter) return;
    Split({
      rowGutters: [
        {
          track: 1,
          element: gutter,
        },
      ],
      minSize: 150,
    });
  });
</script>

<div class="playground">
  <Chat class="grow" {messages} />
  <div bind:this={gutter} class="h-2 cursor-row-resize bg-gray-600"></div>
  <Prompt {onSend} disable={proccessing} />
</div>

<style>
  .playground {
    height: 100vh;
    display: grid;
    /* row heights: chat (flex), gutter (auto), prompt (initial 150px) */
    grid-template-rows: 1fr auto 150px;
  }
</style>
