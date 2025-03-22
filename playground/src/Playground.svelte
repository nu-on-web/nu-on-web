<script lang="ts">
  import {
    runCode as nushellRunCode,
    getCommandsDescriptions as nushellGetCommandsDescriptions,
  } from "./lib/nushell";

  import Prompt from "./components/Prompt.svelte";
  import Chat from "./components/Chat.svelte";
  import type { Message } from "./lib/types";

  let messages = $state<Message[]>([]);

  // async function runCode(code: string, search?: string) {
  //   if (!editor) return;
  //   let codeToRun = code;
  //   if (search) {
  //     codeToRun += ` | find '${search}'`;
  //   }
  //   codeToRun += " | to html -d --partial";
  //   let tmpResult;
  //   try {
  //     const runResult = await nushellRunCode(codeToRun);
  //     if ("success" in runResult) {
  //       tmpResult = runResult.success.String.val;
  //       error = "";
  //     } else if ("parseErrors" in runResult) {
  //       error = "parse errors: " + JSON.stringify(runResult.parseErrors);
  //       return;
  //     } else if ("compileErrors" in runResult) {
  //       error = "compile errors: " + JSON.stringify(runResult.compileErrors);
  //       const decorationCollection = chain(runResult.compileErrors)
  //         .map((error) => {
  //           if ("RunExternalNotFound" in error) {
  //             const range = spanToRange(code, error.RunExternalNotFound.span);
  //
  //             return {
  //               range,
  //               options: {
  //                 inlineClassName:
  //                   "underline decoration-wavy decoration-red-400",
  //                 hoverMessage: {
  //                   value: `Command \`${code.substring(error.RunExternalNotFound.span.start, error.RunExternalNotFound.span.end)}\` not found.`,
  //                 },
  //               },
  //             };
  //           }
  //           return undefined;
  //         })
  //         .compact()
  //         .value();
  //       editor.createDecorationsCollection(decorationCollection);
  //       return;
  //     } else {
  //       error = "results are not clear.";
  //       return;
  //     }
  //   } catch (err) {
  //     console.error(err);
  //     if (err instanceof Error) error = err.message;
  //     return;
  //   }
  //   tmpResult = tmpResult.replaceAll("<ol>", '<ol start="0">');
  //   if (search) {
  //     tmpResult = convert.toHtml(tmpResult);
  //   }
  //   result = tmpResult;
  // }
  //
  // function handleRunClick() {
  //   runCode(code, search);
  // }
  // $effect(() => {
  //   if (autoRun) {
  //     runCode(code, search);
  //   }
  // });
  // function editorReady(
  //   event: CustomEvent<Monaco.editor.IStandaloneCodeEditor>,
  // ) {
  //   editor = event.detail;
  // }
  // let commandsDecoration: Monaco.editor.IEditorDecorationsCollection;
  //
  // async function updateCodeCommandsDecoration(code: string) {
  //   if (!editor) return;
  //   const commandsDescriptions = await nushellGetCommandsDescriptions(code);
  //   console.log({ commandsDescriptions });
  //   return editor.createDecorationsCollection(
  //     commandsDescriptions.map((commandDescription) => {
  //       const range = spanToRange(code, commandDescription.span);
  //       return {
  //         range,
  //         options: { hoverMessage: { value: commandDescription.description } },
  //       };
  //     }),
  //   );
  // }
  // $effect(() => {
  //   console.log(new Date());
  //   if (!editor) return;
  //   commandsDecoration?.clear();
  //   updateCodeCommandsDecoration(code)!.then((v) => {
  //     commandsDecoration = v!; // using ! since we check for editor above;
  //   });
  // });

  function onSend(code: string) {
    messages = [
      ...messages,
      { type: "user", value: code, time: new Date() },
      { type: "nushell", value: "echo 'Hello", time: new Date() },
    ];
  }
</script>

<div class="h-screen flex flex-col">
  <Chat class="grow" {messages} />
  <Prompt {onSend} />
</div>
