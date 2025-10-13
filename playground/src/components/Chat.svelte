<script lang="ts">
  import { Message } from "../lib/types";
  import MessageItem from "./MessageItem.svelte";

  interface Props {
    class?: string;
    messages: Message[];
  }

  let { class: className, messages }: Props = $props();

  let container: HTMLElement;

  $effect(() => {
    messages;
    container.scroll({
      top: container.scrollHeight,
      behavior: "smooth",
    });
  });
</script>

<div
  bind:this={container}
  class="{className} flex flex-col overflow-y-auto p-1 gap-2"
>
  {#each messages as message}
    <MessageItem {message} />
  {/each}
</div>
