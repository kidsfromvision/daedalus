<script lang="ts">
	import type { SvelteHTMLElements } from "svelte/elements";
	type $$Props = SvelteHTMLElements["input"];
</script>

<div
	class="bg-transparent rounded transition duration-100 focus-within:border-gray-400 border border-gray-300 text-md select-none self-stretch sm:self-auto flex flex-row items-center py-1 pr-1 justify-between"
>
	<input
		class="outline-none focus:outline-none ring-0 focus:ring-0 px-3 flex bg-transparent"
		{...$$props}
	/>
	<slot />
</div>

<style>
	input {
		-webkit-tap-highlight-color: rgba(0, 0, 0, 0);
	}
</style>
