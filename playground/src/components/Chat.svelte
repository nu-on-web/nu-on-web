<script lang="ts">
  import { Message } from "../lib/types";
  import dayjs from "dayjs";
  import relativeTime from "dayjs/plugin/relativeTime";
  import MessageItem from "./MessageItem.svelte";
  dayjs.extend(relativeTime);

  interface Props {
    class: string;
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
  class="{className} flex flex-col overflow-y-scroll gap-2"
>
  {#each messages as message}
    <MessageItem {message} />
  {/each}
</div>
