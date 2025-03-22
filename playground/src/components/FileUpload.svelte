<script lang="ts">
  import { promises as fs } from "@zenfs/core";
  import type { FormEventHandler } from "svelte/elements";
  import FileUpload from "~icons/si/file-upload-duotone";

  const handleFileDrop: FormEventHandler<HTMLInputElement> = (e) => {
    const file: File = e.currentTarget.files![0];
    const fileReader = new FileReader();
    fileReader.readAsArrayBuffer(file);
    fileReader.onload = async () => {
      const buf = new Uint8Array(fileReader.result as ArrayBuffer);
      await fs.writeFile("/files/" + file.name, buf);
    };
  };
</script>

<input type="file" onchange={handleFileDrop} class="hidden!" id="upload-file" />
<label for="upload-file" class="btn! btn-primary"
  >Upload file
  <FileUpload />
</label>
