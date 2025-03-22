<script lang="ts">
  import Prompt from "./components/Prompt.svelte";
  import Chat from "./components/Chat.svelte";
  import { runCode } from "./lib/nushell";
  import type { Message } from "./lib/types";

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
</script>

<div class="h-screen flex flex-col">
  <Chat class="grow" {messages} />
  <Prompt {onSend} disable={proccessing} />
</div>
