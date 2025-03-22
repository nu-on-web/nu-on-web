<script lang="ts">
  import Highlight from "svelte-highlight";
  import shell from "svelte-highlight/languages/bash";
  import "svelte-highlight/styles/github-dark.css";
  import sanitizeHtml from "sanitize-html";
  import { Message } from "../lib/types";

  import dayjs from "dayjs";
  import relativeTime from "dayjs/plugin/relativeTime";

  dayjs.extend(relativeTime);

  interface Props {
    message: Message;
  }
  let { message }: Props = $props();

  let currentTime = $state(new Date());

  $effect(() => {
    const interval = setInterval(() => {
      currentTime = new Date();
    }, 60000);
    return () => clearInterval(interval);
  });

  let time = $derived(dayjs(message.time).from(currentTime));
</script>

<div class="chat {message.type === 'user' ? 'chat-end' : 'chat-start'}">
  <div class="chat-header">
    {message.type}
  </div>
  <div
    class="chat-bubble {message.type === 'user'
      ? 'chat-bubble-primary'
      : 'chat-bubble-secondary'}"
  >
    {#if message.type === "user"}
      <Highlight language={shell} code={message.value}></Highlight>
    {:else if "success" in message.value}
      {@html sanitizeHtml(message.value.success.String.val)}
    {:else}
      {JSON.stringify(message.value)}
    {/if}
  </div>
  <div class="chat-footer text-xs opacity-50">
    {time}
  </div>
</div>
