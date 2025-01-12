<script lang="ts">
	import '../app.css';
	let { children } = $props();

	import { Toaster } from 'svelte-sonner';
	import '../app.css';
	import ImageDialogItem from '$lib/components/ImageDialogItem.svelte';
	import { selectedImage, isNightMode } from '$lib';
	import Feedback from '$lib/components/Feedback.svelte';
</script>

<svelte:head>
	{#if !$isNightMode}
		<meta name="theme-color" content="#FFFFFF" />
	{:else}
		<meta name="theme-color" content="#000000" />
	{/if}
</svelte:head>
<div class="relative flex flex-col items-center justify-center bg-[#f9fafb]">
	<div class="bgLight absolute left-0 top-0 h-screen w-full"></div>
	<div class="z-20 flex flex-col items-center justify-start pb-8">
		{@render children()}

		<ImageDialogItem></ImageDialogItem>
		<Feedback></Feedback>
		<Toaster></Toaster>
	</div>
</div>
