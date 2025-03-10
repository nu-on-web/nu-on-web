<script lang="ts">
  import {fs} from "@zenfs/core"
  import Dropzone from "svelte-file-dropzone";
  import MonacoEditor from "svelte-monaco";
  import * as Monaco from "monaco-editor";
  import Convert from "ansi-to-html";
  let convert = new Convert();
  let editor: Monaco.editor.IStandaloneCodeEditor | null = $state(null);
  import { run_code, get_commands_descriptions } from "./wasm/nushell_wasm";
  import { Result, GetCommandsDescriptionsResult } from "./lib/types";
  import { spanToRange } from "./lib/utils";
  import { chain } from "lodash-es";

  let result: string = $state("");

  let code = $state("ls");
  let search = $state("");
  let autoRun = $state(false);
  let error = $state("");
  async function handleFileDrop(e: CustomEvent<any>) {
    const file: File = e.detail.acceptedFiles[0];
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
    if (!editor) return;
    let codeToRun = code;
    if (search) {
      codeToRun += ` | find '${search}'`;
    }
    codeToRun += " | to html -d --partial";
    let tmpResult;
    try {
      const runOutput = run_code(codeToRun);
      const runResult = Result.parse(runOutput);
      if ("success" in runResult) {
        tmpResult = runResult.success.String.val;
        error = "";
      } else if ("parseErrors" in runResult) {
        error = "parse errors: " + JSON.stringify(runResult.parseErrors);
        return;
      } else if ("compileErrors" in runResult) {
        error = "compile errors: " + JSON.stringify(runResult.compileErrors);
        const decorationCollection = chain(runResult.compileErrors)
          .map((error) => {
            if ("RunExternalNotFound" in error) {
              const range = spanToRange(code, error.RunExternalNotFound.span);

              return {
                range,
                options: {
                  inlineClassName:
                    "underline decoration-wavy decoration-red-400",
                  hoverMessage: {
                    value: `Command \`${code.substring(error.RunExternalNotFound.span.start, error.RunExternalNotFound.span.end)}\` not found.`,
                  },
                },
              };
            }
            return undefined;
          })
          .compact()
          .value();
        editor.createDecorationsCollection(decorationCollection);
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
    if (autoRun) {
      runCode(code, search);
    }
  });
  function editorReady(
    event: CustomEvent<Monaco.editor.IStandaloneCodeEditor>,
  ) {
    editor = event.detail;
  }
  let commandsDecoration: Monaco.editor.IEditorDecorationsCollection;

  function updateCodeCommandsDecoration(code: string) {
    if (!editor) return;
    const getCommandsDescriptionsResult = get_commands_descriptions(code);
    const commandsDescriptions = GetCommandsDescriptionsResult.parse(
      getCommandsDescriptionsResult,
    );
    console.log({ commandsDescriptions });
    return editor.createDecorationsCollection(
      commandsDescriptions.map((commandDescription) => {
        const range = spanToRange(code, commandDescription.span);
        return {
          range,
          options: { hoverMessage: { value: commandDescription.description } },
        };
      }),
    );
  }
  $effect(() => {
    console.log(new Date());
    if (!editor) return;
    commandsDecoration?.clear();
    commandsDecoration = updateCodeCommandsDecoration(code)!; // using ! since we check for editor above
  });
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
