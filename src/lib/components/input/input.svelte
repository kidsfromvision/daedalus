<script lang="ts">
	export let placeholder: string;
</script>

<div
	class="bg-white focus-within:bg-gray-50 transition rounded duration-100 border border-gray-300 text-md select-none self-stretch sm:self-auto flex flex-row items-center py-1 pr-1 justify-between"
>
	<input
		class="outline-none focus:outline-none ring-0 focus:ring-0 px-3 flex bg-transparent"
		{placeholder}
	/>
	<slot />
</div>

<style>
	input {
		-webkit-tap-highlight-color: rgba(0, 0, 0, 0);
	}
</style>
