<script lang="ts">
	import ButtonLight from '$lib/components/button/button-light.svelte';
	import ButtonDark from '$lib/components/button/button-dark.svelte';
	import ButtonIcon from '$lib/components/button/button-icon.svelte';
	import Input from '$lib/components/input/input.svelte';
</script>

<div class="flex flex-col items-center gap-y-8 py-16 px-2 sm:px-16 rounded-lg">
	<h1>Components</h1>
	<ButtonLight>Light Button</ButtonLight>
	<ButtonDark>Dark Button</ButtonDark>
	<Input placeholder="Placeholder">
		<ButtonIcon>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 20 20"
				fill="currentColor"
				class="w-5 h-5"
			>
				<path
					fill-rule="evenodd"
					d="M3 10a.75.75 0 01.75-.75h10.638L10.23 5.29a.75.75 0 111.04-1.08l5.5 5.25a.75.75 0 010 1.08l-5.5 5.25a.75.75 0 11-1.04-1.08l4.158-3.96H3.75A.75.75 0 013 10z"
					clip-rule="evenodd"
				/>
			</svg>
		</ButtonIcon>
	</Input>
</div>
