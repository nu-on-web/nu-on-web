<script lang="ts">
  import Convert from "ansi-to-html";
  import Error from "~icons/si/dangerous-duotone";
  import sanitizeHtml from "sanitize-html";
  import Highlight from "svelte-highlight";
  import shell from "svelte-highlight/languages/bash";
  import "svelte-highlight/styles/github-dark.css";
  import type { Message } from "../lib/types";

  import dayjs from "dayjs";
  import duration from "dayjs/plugin/duration";
  import relativeTime from "dayjs/plugin/relativeTime";

  dayjs.extend(relativeTime);
  dayjs.extend(duration);

  interface Props {
    message: Message;
  }
  let { message }: Props = $props();

  const convert = new Convert();

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
  {#if message.type === "user"}
    <div class="chat-bubble bg-slate-950 before:bg-slate-950 max-h-[80vh]">
      <Highlight language={shell} code={message.value} class="shadow-xl" />
    </div>
  {:else if message.value.type === "success" && message.value.vtype === "string"}
    <div class="chat-bubble bg-slate-950 before:bg-slate-950 max-h-[80vh]">
      <div class="response-content">
        {@html convert.toHtml(sanitizeHtml(message.value.val))}
      </div>
    </div>
  {:else if "error" in message.value}
    <div
      class="chat-bubble max-h-[80vh] bg-error text-error-content flex items-center gap-2"
    >
      <Error class="h-5 w-5" />
      <div class="response-content">
        {message.value.error.msg}
      </div>
    </div>
  {:else}
    <div class="chat-bubble bg-slate-950 before:bg-slate-950 max-h-[80vh]">
      {JSON.stringify(message.value)}
    </div>
  {/if}
  <div class="chat-footer text-xs opacity-50">
    {time}
  </div>
</div>

<style global>
  @reference "tailwindcss";
  :global(.response-content div) {
    @apply text-gray-200 font-mono whitespace-pre-wrap;
  }

  :global(.response-content ol) {
    @apply max-w-md space-y-1 list-decimal list-inside p-1;
    list-style: none;
  }

  :global(.response-content ol li) {
    margin: 0;
  }
  :global(.response-content ol li:not(:first-child)) {
    counter-increment: list-counter 1;
  }
  :global(.response-content ol li)::before {
    content: counter(list-counter) " | ";
    @apply font-bold text-gray-600;
  }

  :global(.response-content table) {
    @apply table-auto text-sm text-left text-gray-400 shadow-xl;
  }

  :global(.response-content thead) {
    @apply text-xs bg-gray-600 text-gray-400;
  }

  :global(.response-content thead th) {
    @apply px-6 py-3;
  }

  :global(.response-content thead th:nth-child(even)) {
    @apply bg-gray-700;
  }

  :global(.response-content tbody tr) {
    @apply border-b bg-gray-800 border-gray-700;
  }

  :global(.response-content tbody td) {
    @apply px-6 py-4;
  }

  :global(.response-content tbody td:nth-child(even)) {
    @apply bg-gray-900;
  }
</style>
