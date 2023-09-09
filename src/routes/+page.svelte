<script>
	import Modal from '$lib/components/modal.svelte';
	let showModal = false;
</script>

<button on:click={() => (showModal = true)}>Open Modal</button>

<Modal bind:isOpen={showModal}>
	<div class="w-full h-full mt-2">
		<h1 class="text-xl font-bold mb-4">Modal Title</h1>
		<p>This is the modal content.</p>
	</div>
</Modal>
