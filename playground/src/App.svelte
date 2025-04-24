<script lang="ts">
  import "./app.css";
  import Playground from "./Playground.svelte";
  import { configureSingle } from "@zenfs/core";
  import { mkdir, writeFile, exists } from "@zenfs/core/promises";
  import { WebStorage } from "@zenfs/dom";

  let fsSetupPromise = (async () => {
    await configureSingle({ backend: WebStorage });
    await mkdir("/files", { recursive: true });
    if (!(await exists("/files/example.json")))
      await writeFile("/files/example.json", "[{v:1},{v:2},{v:3}]");
  })().catch((error) =>
    console.error("Error in setting up the filesystem:", error),
  );
</script>

{#await fsSetupPromise}
  <div class="flex items-center justify-center h-screen">
    <span class="loading loading-spinner loading-lg"></span>
  </div>
{:then}
  <Playground />
{/await}

<!-- <Playground /> -->
