<script lang="ts">
  import { Message } from "../lib/types";
  import prism from "prismjs";
  import "prismjs/themes/prism-dark.css";

  interface Props {
    message: Message;
  }
  let { message }: Props = $props();

  let value = $derived(
    // prism.highlight(message.value, prism.languages.shell, "shell")
    prism.highlight(message.value, prism.languages.javascript, "javascript"),
  );
</script>

<div class="chat {message.type === 'user' ? 'chat-end' : 'chat-start'}">
  <div class="chat-header">
    {message.type}
    <!-- <time class="text-xs opacity-50">{dayjs(message.time).diff(dayjs())}</time> -->
  </div>
  <div
    class="chat-bubble {message.type === 'user'
      ? 'chat-bubble-primary'
      : 'chat-bubble-secondary'}"
  >
    {@html value}
  </div>
</div>
