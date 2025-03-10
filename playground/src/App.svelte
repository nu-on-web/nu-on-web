<script lang="ts">
  import { onMount } from "svelte";
  import Playground from "./Playground.svelte";
  import init from "./wasm/nushell_wasm";
  import { fs } from "@zenfs/core"

  let ready = false;
  onMount(() => {
    init().then(() => {
      fs.writeFileSync("./test.json", "[1,2,3]");
      ready = true;
    });
  });
</script>

{#if ready}
  <Playground />
{/if}
