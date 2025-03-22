<script lang="ts">
  import "./app.css";
  import Playground from "./Playground.svelte";
  import { fs } from "@zenfs/core";

  $effect(() => {
    fs.mkdirSync("/files");
    fs.writeFileSync("/files/example.json", "[{v:1},{v:2},{v:3}]");
  });
</script>

<Playground />
