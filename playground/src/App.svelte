<script lang="ts">
  import "./app.css";
  import Playground from "./Playground.svelte";
  import init from "./wasm/nushell_wasm";
  import { fs } from "@zenfs/core";

  $effect(() => {
    init().then(() => {
      fs.writeFileSync("./test.json", "[1,2,3]");
    });
  });
</script>

<Playground />
