<script lang="ts">
  import * as BrowserFS from "browserfs";
  import Dropzone from "svelte-file-dropzone";
  import MonacoEditor from "svelte-monaco";
  import * as Monaco from "monaco-editor";
  import Convert from "ansi-to-html";
  let convert = new Convert();
  let editor: Monaco.editor.IStandaloneCodeEditor;
  import { run_code } from "./wasm/nushell_wasm";
  import { Result } from "./lib/types";
  import lineColumn from "line-column";
  import { chain } from "lodash-es";

  let result: string = $state("");

  let code = $state("ls");
  let search = $state("");
  let autoRun = $state(false);
  let error = $state("");
  async function handleFileDrop(e: CustomEvent<any>) {
    console.log("detail:", e.detail);
    const file: File = e.detail.acceptedFiles[0];
    console.log({ file });
    const fs = BrowserFS.BFSRequire("fs");
    fs.writeFileSync(file.name, Buffer.from([]));
    await file.stream().pipeTo(
      new WritableStream({
        write({ buffer }) {
          fs.appendFileSync(file.name, Buffer.from(buffer));
        },
      }),
    );
  }
  function runCode(code: string, search?: string) {
    let codeToRun = code;
    if (search) {
      codeToRun += ` | find '${search}'`;
    }
    codeToRun += " | to html -d --partial";
    let tmpResult;
    try {
      const runOutput = JSON.parse(run_code(codeToRun));
      console.log({ runOutput });
      const runResult = Result.parse(runOutput);
      if ("success" in runResult) {
        tmpResult = runResult.success.String.val;
        error = "";
      } else if ("parseErrors" in runResult) {
        error = "parse errors: " + JSON.stringify(runResult.parseErrors);
        return;
      } else if ("compileErrors" in runResult) {
        error = "compile errors: " + JSON.stringify(runResult.compileErrors);
        console.log(123, editor.getModel());
        Monaco.editor.setModelMarkers(
          editor.getModel()!,
          "nu-errors",
          chain(runResult.compileErrors)
            .map((error) => {
              if ("RunExternalNotFound" in error) {
                const { line: startLineNumber, col: startColumn } = lineColumn(
                  code,
                ).fromIndex(error.RunExternalNotFound.span.start)!;
                const { line: endLineNumber, col: endColumn } = lineColumn(
                  code,
                ).fromIndex(
                  Math.min(code.length - 1, error.RunExternalNotFound.span.end),
                )!;
                console.log({
                  endLineNumber,
                  endColumn,
                  startColumn,
                  startLineNumber,
                });
                return {
                  severity: Monaco.MarkerSeverity.Error,
                  message: "Command Not found",
                  startLineNumber,
                  startColumn,
                  endLineNumber,
                  endColumn,
                } satisfies Monaco.editor.IMarkerData;
              }
              return undefined;
            })
            .compact()
            .value(),
        );
        return;
      } else {
        error = "results are not clear.";
        return;
      }
    } catch (err) {
      console.error(err);
      if (err instanceof Error) error = err.message;
      return;
    }
    tmpResult = tmpResult.replaceAll("<ol>", '<ol start="0">');
    if (search) {
      tmpResult = convert.toHtml(tmpResult);
    }
    result = tmpResult;
  }

  function handleRunClick() {
    runCode(code, search);
  }
  $effect(() => {
    console.log(new Date());
    if (autoRun) {
      runCode(code, search);
    }
  });
  function editorReady(
    event: CustomEvent<Monaco.editor.IStandaloneCodeEditor>,
  ) {
    console.log("editor is ready!");
    editor = event.detail;
    Monaco.editor.onDidChangeMarkers((e) => console.log(55, e));
  }
</script>

<div class="min-h-full bg-black">
  <Dropzone on:drop={handleFileDrop} />
  <div class="h-16">
    <MonacoEditor
      options={{ language: "shell" }}
      theme="vs-dark"
      bind:value={code}
      on:ready={editorReady}
    />
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
      class="bg-green-300 my-2 hover:bg-green-400 py-1 px-4 cursor-pointer transition duretion-100 disabled:pointer-events-none disabled:bg-gray-700"
      onclick={handleRunClick}
      disabled={autoRun}
    >
      RUN
    </button>
    <div class="flex items-center">
      <input
        id="auto_run"
        class="w-4 h-4 text-blue-600 rounded-sm focus:ring-blue-600 ring-offset-gray-800 focus:ring-2 bg-gray-700 border-gray-600"
        type="checkbox"
        bind:checked={autoRun}
      />
      <label class="ms-2 text-sm font-medium text-gray-300" for="auto_run"
        >Auto RUN</label
      >
    </div>
    <div class="mt-1">
      {#if error}
        <span class="text-red-600">{error}</span>
      {/if}
      {#if result}
        {@html result}
      {:else}
        <i>No Result</i>
      {/if}
    </div>
  </div>
</div>
