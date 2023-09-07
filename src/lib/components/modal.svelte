<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';

	const dispatch = createEventDispatcher();

	export let isOpen: boolean = false;
	export let showCloseButton: boolean = true;
	export let submitLabel: string | null = null;

	const zIndex = 50; // Fixed z-index

	onMount(() => {
		document.addEventListener('keydown', handleKeyDown);
		return () => {
			document.removeEventListener('keydown', handleKeyDown);
		};
	});

	function handleKeyDown(event: KeyboardEvent): void {
		if (event.key === 'Escape') {
			close();
		}
		// Add further key-based handlers if needed.
	}

	function close(): void {
		isOpen = false;
		dispatch('close');
	}

	function submit(): void {
		dispatch('submit');
	}

	function handleBackgroundKeyDown(event: KeyboardEvent): void {
		if (event.key === 'Enter') {
			close();
		}
	}
</script>

<div
	class="{isOpen ? 'fixed' : 'hidden'} inset-0 flex items-center justify-center z-{zIndex}"
	style="background-color: rgba(0,0,0,0.5);"
	on:click={close}
	on:keydown={handleBackgroundKeyDown}
	role="button"
	tabindex="0"
>
	<div
		class="bg-white rounded-lg shadow-md max-w-lg w-full p-6 relative overflow-auto max-h-screen"
		on:click|stopPropagation
		on:keydown={handleBackgroundKeyDown}
		role="button"
		tabindex="0"
	>
		{#if showCloseButton}
			<button class="absolute top-2 right-2" on:click={close}>X</button>
		{/if}
		<slot />
		{#if submitLabel}
			<button on:click={submit} class="mt-4 px-4 py-2 bg-blue-500 text-white rounded"
				>{submitLabel}</button
			>
		{/if}
	</div>
</div>

<style>
	/* Add media queries for mobile responsiveness */
	@media (max-width: 640px) {
		/* Here we make the modal almost full screen on small devices */
		.bg-white {
			margin: 0.5rem;
			height: calc(100% - 1rem);
		}
	}
</style>
