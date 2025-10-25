<script lang="ts">
  import PlayIcon from "~icons/mdi/play";
  import type { Message } from "../lib/types";
  import { code } from "../lib/stores/code";

  interface Props {
    message: Extract<Message, { type: "user" }>["value"];
  }
  let { message }: Props = $props();
</script>

<button
  class="clickable-area"
  onclick={() => code.set(message)}
  aria-label="Replay this command"
>
  <PlayIcon class="inline-block w-7 h-7 mr-1" />
</button>

<style>
  .clickable-area:hover {
    cursor: pointer;
    align-items: baseline;
  }
</style>
