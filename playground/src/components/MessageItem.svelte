<script lang="ts">
  import Highlight from "svelte-highlight";
  import shell from "svelte-highlight/languages/bash";
  import "svelte-highlight/styles/github-dark.css";
  import sanitizeHtml from "sanitize-html";
  import { Message } from "../lib/types";

  interface Props {
    message: Message;
  }
  let { message }: Props = $props();
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
</div>
