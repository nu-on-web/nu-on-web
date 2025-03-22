<script lang="ts">
  import Highlight from "svelte-highlight";
  import shell from "svelte-highlight/languages/bash";
  import sanitizeHtml from "sanitize-html";
  import "svelte-highlight/styles/github-dark.css";
  import { Message } from "../lib/types";

  import dayjs from "dayjs";
  import relativeTime from "dayjs/plugin/relativeTime";
  import duration from "dayjs/plugin/duration";

  dayjs.extend(relativeTime);
  dayjs.extend(duration);

  interface Props {
    message: Message;
  }
  let { message }: Props = $props();

  let currentTime = $state(new Date());

  $effect(() => {
    const interval = setInterval(() => {
      currentTime = new Date();
    }, dayjs.duration(1, "minutes").asMilliseconds());
    return () => clearInterval(interval);
  });

  let time = $derived(dayjs(message.time).from(currentTime));
</script>

<div class="chat {message.type === 'user' ? 'chat-end' : 'chat-start'}">
  <div class="chat-header">
    {message.type}
  </div>
  <div class="chat-bubble bg-slate-950 before:bg-slate-950">
    {#if message.type === "user"}
      <Highlight language={shell} code={message.value} class="shadow-xl" />
    {:else if "success" in message.value}
      <div class="response-content">
        {@html sanitizeHtml(message.value.success.String.val)}
      </div>
    {:else}
      {JSON.stringify(message.value)}
    {/if}
  </div>
  <div class="chat-footer text-xs opacity-50">
    {time}
  </div>
</div>

<style lang="scss">
  @import "tailwindcss";

  :global .response-content {
    div {
      background-color: transparent !important;
      @apply text-gray-200!;
    }

    ol {
      @apply max-w-md space-y-1 list-decimal list-inside p-1;
      list-style: none;
      li {
        margin: 0 !important;
        &:not(:first-child) {
          counter-increment: list-counter 1;
        }
        &::before {
          content: counter(list-counter) " | ";
          @apply font-bold text-gray-600;
        }
      }
    }

    table {
      @apply table-auto text-sm text-left text-gray-400 shadow-xl;
    }

    thead {
      @apply text-xs bg-gray-600 text-gray-400;
      th {
        @apply px-6 py-3;
        &:nth-child(even) {
          @apply bg-gray-700;
        }
      }
    }

    tbody {
      tr {
        @apply border-b bg-gray-800 border-gray-700;
      }
      td {
        @apply px-6 py-4;
        &:nth-child(even) {
          @apply bg-gray-900;
        }
      }
    }
  }
</style>
