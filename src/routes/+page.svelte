<script lang="ts">
	import { open } from "@tauri-apps/api/dialog";
	import { invoke } from "@tauri-apps/api/tauri";
    import Modal  from './Modal.svelte';

    let showModal: boolean = false;

    function openModal() {
        showModal = true;
    }

    let selectedFilePath: string | string[] | null;
    let msgAfterConvertedVideo: string;


    async function dialogPickFile() {
        const selected = await open({
          multiple: false,
          filters: [{
            name: 'Video',
            extensions: ['mp4']
          }]
        });
        selectedFilePath = selected;

        if(selectedFilePath !== null) {
            console.log("j: ", selected);
        }
    }

    async function onFileSelectedRS() {
        if(selectedFilePath != null) {
            msgAfterConvertedVideo = await invoke('upload_file', { path: selectedFilePath });
            openModal();
        }
    }
</script>

<div class="flex h-screen">
    <div class="m-auto">
        <h1 class="text-xl font-bold">Welcome to giffppp</h1>
        <button on:click={dialogPickFile} class="btn btn-outline btn-accent">Pick File</button>
        <button on:click="{onFileSelectedRS}" class="btn btn-primary">Submit</button>
    </div>
</div>

<Modal bind:showModal msgDisplay={ msgAfterConvertedVideo } />
