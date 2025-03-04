<script lang="ts">
  import Monaco from "svelte-monaco";
  import Convert from "ansi-to-html";
  let convert = new Convert();
  import { run_code } from "./wasm/nushell_wasm";
  let result: string = "";

  let code = "ls";
  let search = "";
  function runCode() {
    let codeToRun = code;
    if (search) {
      codeToRun += ` | find '${search}'`;
    }
    codeToRun += " | to html -d --partial";
    let tmpResult = JSON.parse(run_code(codeToRun)).String.val;
    tmpResult = tmpResult.replaceAll("<ol>", '<ol start="0">');
    if (search) {
      tmpResult = convert.toHtml(tmpResult);
    }
    result = tmpResult;
  }
  $: console.log("Result:", result);
</script>

<div class="min-h-full bg-black">
  <div class="h-16">
    <Monaco options={{ language: "shell" }} theme="vs-dark" bind:value={code} />
  </div>
  <div class="pl-[68px]">
    <div class="my-2 flex items-center gap-2">
      <label class="text-white" for="search">Search:</label>
      <input
        id="search"
        class="block text-white border border-white focus:border-gray-200 outline-0"
        placeholder="Search..."
        bind:value={search}
      />
    </div>
    <button
      class="bg-green-300 my-2 hover:bg-green-400 py-1 px-4 cursor-pointer transition duretion-100"
      on:click={runCode}>RUN</button
    >
    <hr />
    <div>
      {#if result}
        {@html result}
      {:else}
        <i>No Result</i>
      {/if}
    </div>
  </div>
</div>
