<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import { fly, fade } from 'svelte/transition';

	const dispatch = createEventDispatcher();

	export let isOpen: boolean = false;
	export let showCloseButton: boolean = true;
	export let submitLabel: string | null = null;

	const zIndex = 50; // Fixed z-index

	onMount(() => {
		if (typeof document !== 'undefined') {
			document.addEventListener('keydown', handleKeyDown);
			// this is so you can't scroll the background when the modal is open
			if (isOpen) {
				document.body.style.overflow = 'hidden';
			} else {
				document.body.style.overflow = '';
			}
			return () => {
				document.removeEventListener('keydown', handleKeyDown);
				document.body.style.overflow = '';
			};
		}
	});

	$: if (typeof document !== 'undefined') {
		if (isOpen) {
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
		}
	}

	function handleKeyDown(event: KeyboardEvent): void {
		if (event.key === 'Escape') {
			close();
		}
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
	let yValue: number;

	if (typeof window !== 'undefined') {
		yValue = window.innerHeight;
	}
</script>

{#if isOpen}
	<div
		class="fixed inset-0 flex items-center justify-center z-{zIndex} bg-black bg-opacity-50"
		on:click={close}
		on:keydown={handleBackgroundKeyDown}
		role="button"
		tabindex="0"
		in:fade={{ duration: 150 }}
		out:fade={{ duration: 150 }}
	>
		<div
			class="bg-white rounded-lg shadow-md max-w-lg w-full p-6 relative max-h-screen"
			on:click|stopPropagation
			on:keydown={handleBackgroundKeyDown}
			role="button"
			tabindex="0"
			in:fly={{ y: yValue, duration: 350 }}
			out:fly={{ y: yValue, duration: 200 }}
		>
			{#if showCloseButton}
				<button class="absolute top-2 right-2" on:click={close}>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="1.5"
						stroke="currentColor"
						class="w-6 h-6"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			{/if}
			<div class="overflow-auto h-full">
				<slot />
				{#if submitLabel}
					<button on:click={submit} class="mt-4 px-4 py-2 bg-blue-500 text-white rounded">
						{submitLabel}
					</button>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	@media (max-width: 640px) {
		.bg-white {
			margin-top: 4rem;
			width: 100%;
			height: calc(100% - 4rem);
			border-radius: 0.5rem 0.5rem 0 0;
			overflow-y: auto; /* This ensures your modal is scrollable if the content exceeds its height */
		}
	}
</style>
