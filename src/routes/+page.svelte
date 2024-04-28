<script lang="ts">

    let name: string = 'world';
    let inputVideo: File;

    const getFileInput = (fileInput: File) => () => {
        if(fileInput != undefined) {
            alert(`Hello, ${fileInput.name}!`);
        }
    }
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
  <input accept="video/mp4" on:change={onFileSelected} id="avatarEx" name="avatarEx" type="file" class="file-input file-input-bordered w-full max-w-xs" />
  <div class="label">
    <span class="label-text-alt">Alt label</span>
    <span class="label-text-alt">Alt label</span>
  </div>
</label>

<button on:click="{getFileInput(inputVideo)}" class="btn btn-primary">Submit</button>
