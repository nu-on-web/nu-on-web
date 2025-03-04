<script lang="ts">
  import { onMount } from "svelte";
  import Playground from "./Playground.svelte";
  import init from "./wasm/nushell_wasm";
  import * as BrowserFS from "browserfs";

  let ready = false;
  onMount(() => {
    BrowserFS.install(window);
    BrowserFS.configure({ fs: "LocalStorage", options: {} }, (e) => {
      if (e) {
        throw e;
      }
      init().then(() => {
        ready = true;
        const fs = BrowserFS.BFSRequire("fs");
        fs.writeFileSync("./test.json", "[1,2,3]");
      });
    });
  });
</script>

{#if ready}
  <Playground />
{/if}
