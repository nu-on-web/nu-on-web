<script lang="ts">
  import "./app.css";
  import Playground from "./Playground.svelte";
  import { configureSingle } from "@zenfs/core";
  import { mkdir, writeFile, exists } from "@zenfs/core/promises";
  import { WebStorage } from "@zenfs/dom";

  let v = configureSingle({ backend: WebStorage })
    .then(() => mkdir("/files", { recursive: true }))
    .then(() => exists("/files/example.json"))
    .then((v) =>
      v ? writeFile("/files/example.json", "[{v:1},{v:2},{v:3}]") : undefined,
    );
</script>

{#await v}
  <div class="flex items-center justify-center h-screen">
    <span class="loading loading-spinner loading-lg"></span>
  </div>
{:then}
  <Playground />
{/await}

<!-- <Playground /> -->
