<script lang="ts">
  import FileUpload from "./FileUpload.svelte";
  import { promises as fs, watch } from "@zenfs/core";
  import byteSize from "byte-size";
  import type { MouseEventHandler } from "svelte/elements";
  import Remove from "~icons/si/remove-square-duotone";

  interface Props {
    class?: string;
    onFileClick: (filename: string) => void;
  }

  let { class: className, onFileClick }: Props = $props();

  let filesPromise = $state<Promise<{ name: string; size: number }[]>>();
  async function getFiles() {
    const files = await fs.readdir("/files");
    return await Promise.all(
      files.map(async (file) => {
        const stat = await fs.stat("/files/" + file);
        return {
          name: file,
          size: stat.size,
        };
      }),
    );
  }

  $effect(() => {
    filesPromise = getFiles();
    watch("/files", () => {
      filesPromise = getFiles();
    });
  });

  const handleFileClick: MouseEventHandler<HTMLButtonElement> = (e) => {
    const filename = e.currentTarget?.dataset.filename;
    if (filename) onFileClick(filename);
  };
</script>

<aside class="flex items-center {className}">
  <FileUpload />
  {#if filesPromise}
    {#await filesPromise}
      <span>Loading files...</span>
    {:then files}
      <ul class="p-2 gap-2 flex items-center">
        {#each files as file}
          {@const fileSize = byteSize(file.size)}
          <li class="indicator">
            <button
              data-filename={file.name}
              class="indicator-item badge bg-transparent border-0"
              onclick={(e) => {
                const filename = e.currentTarget?.dataset.filename;
                fs.unlink(`/files/${filename}`);
              }}><Remove class="hover:opacity-60 cursor-pointer" /></button
            >
            <div class="w-full tooltip tooltip-secondary">
              <div class="tooltip-content w-fit min-w-full">
                <code>
                  {file.name}
                </code>
                <span class="font-semibold opacity-60"
                  >- {fileSize.value}{fileSize.unit}</span
                >
              </div>
              <button
                class="btn justify-start w-full btn-soft btn-sm whitespace-nowrap overflow-hidden text-ellipsis"
                data-filename={file.name}
                onclick={handleFileClick}
              >
                <span class="overflow-hidden text-ellipsis">
                  {file.name}
                </span>
              </button>
            </div>
          </li>
        {/each}
      </ul>
    {:catch}
      <span>Failed to load files </span>
    {/await}
  {/if}
</aside>
