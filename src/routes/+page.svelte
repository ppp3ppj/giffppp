<script lang="ts">
	import { message, open } from "@tauri-apps/api/dialog";
	import { invoke } from "@tauri-apps/api/tauri";
    import Modal  from './Modal.svelte';
	import { onMount } from "svelte";

    let showModal: boolean = false;

    function openModal() {
        showModal = true;
        console.log("Result Modal is: ", showModal);
    }

    let name: string = 'world';
    let inputVideo: File;
    let selectedFilePath: string | string[] | null;
    let msgAfterConvertedVideo: string;

    const getFileInput = async (fileInput: File) => async () => {
        if(fileInput != undefined) {
            const stringName: string = fileInput.name;
        }
    }

    async function dialogPickFile() {
        const selected = await open({
          multiple: false,
          filters: [{
            name: 'Video',
            extensions: ['mp4']
          }]
        });
        selectedFilePath = selected;

        if(selectedFilePath != null) {
            console.log("j: ", selected);
        }
    }

    async function onFileSelectedRS() {
        if(selectedFilePath != null) {
            msgAfterConvertedVideo = await invoke('upload_file', { path: selectedFilePath });
            openModal();
        }
    }

    const handleIsTauri = () => {
        return Boolean(
      typeof window !== 'undefined' &&
      window !== undefined &&
      window.__TAURI_IPC__ !== undefined
    )};

    function onFileSelected(event: Event) {
        const target = event.target as unknown as { files: File[] };
        const file: File = target?.files[0];
        inputVideo = file;
        console.log(file, target);
        console.log("file is: ", inputVideo);
    }
</script>
<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<label class="form-control w-full max-w-xs">
  <div class="label">
    <span class="label-text">Pick a file</span>
    <span class="label-text-alt">Alt label</span>
  </div>
  <input placeholder="Enter file." bind:value={inputVideo}  accept="video/mp4" on:change={onFileSelected} id="avatarEx" name="avatarEx" type="file" class="file-input file-input-bordered w-full max-w-xs" />
  <div class="label">
    <span class="label-text-alt">Alt label</span>
    <span class="label-text-alt">Alt label</span>
  </div>
</label>
<button on:click={dialogPickFile} class="btn btn-outline btn-accent">Pick File</button>

<button on:click="{onFileSelectedRS}" class="btn btn-primary">Submit</button>

<Modal bind:showModal msgDisplay={ msgAfterConvertedVideo } />
