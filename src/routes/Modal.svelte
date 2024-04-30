<script lang="ts">
 export let showModal: boolean = false; // boolean
 export let msgDisplay: string;

 let dialog: HTMLDialogElement;
 $: if (showModal) dialog.showModal();

</script>

<dialog
	bind:this={dialog}
	on:close={() => (showModal = false)}
	on:click|self={() => dialog.close()}
    class="modal"
>
  <div class="modal-box">
    <h3 class="font-bold text-lg">Alert!</h3>
    <p class="py-4">{ msgDisplay }</p>
  </div>
  <form method="dialog" class="modal-backdrop">
    <button>close</button>
</dialog>
