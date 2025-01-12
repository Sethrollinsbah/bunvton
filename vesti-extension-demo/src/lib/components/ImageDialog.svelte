<script lang="ts">
	import { blur } from 'svelte/transition';
	import { Eye, RefreshCcw } from 'lucide-svelte';
	import { Button, buttonVariants } from './ui/button';
	import * as Dialog from './ui/dialog';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	let placeholder: typeof import('./svg/Placeholder.svelte').default | null = null;

	onMount(async () => {
		// Dynamically import the SVG component
		const { default: PlaceholderSvg } = await import('./svg/Placeholder.svelte');
		placeholder = PlaceholderSvg;
	});
	let nativeLoading = false;
	// Determine whether to bypass our intersecting check
	onMount(() => {
		if ('loading' in HTMLImageElement.prototype) {
			nativeLoading = true;
		}
	});
	export let img = '/placeholder.svg';
	let loaded = false;
	let spin: boolean = false;
	let exists = false;

	async function checkImage(url) {
		spin = true;
		return new Promise((resolve) => {
			if (browser) {
				spin = false;
				const img = new Image();
				img.onload = () => resolve(true); // Image loaded successfully
				img.onerror = () => resolve(false); // Error occurred while loading
				img.src = url;
			}
		});
	}

	// Function to update `exists`
	async function updateImageExists() {
		if (img) {
			exists = await checkImage(img);
		}
	}

	// Trigger update whenever `result` changes
	$: if (img) {
		updateImageExists();
	}
</script>

{#if exists}
	<Dialog.Root>
		<div class="overflow-clip rounded-sm">
			{#if img && nativeLoading}
				<div class="group relative aspect-video h-fit w-full">
					<img
						in:blur={{ duration: 300 }}
						src={img}
						class="h-full w-full object-cover object-center grayscale transition-all duration-300 hover:grayscale-0"
						loading="lazy"
					/>
					<div
						class="absolute left-0 top-0 h-full w-full backdrop-blur-sm transition-all duration-100 ease-in group-hover:backdrop-blur-md"
					></div>

					<Dialog.Trigger asChild let:builder>
						<Button
							class="absolute bottom-2 right-2 translate-y-16 scale-0 opacity-0 transition-all duration-100 group-hover:translate-y-0 group-hover:scale-100 group-hover:opacity-100"
							builders={[builder]}
							size="icon"
							variant="outline"
						>
							<Tooltip.Root>
								<Tooltip.Trigger>
									<Eye class="size-4 text-muted-foreground" />
								</Tooltip.Trigger>
								<Tooltip.Content>
									<p>View generation</p>
								</Tooltip.Content>
							</Tooltip.Root></Button
						></Dialog.Trigger
					>
				</div>
			{:else if placeholder}
				<div class="group relative aspect-video h-fit w-full">
					<svelte:component this={placeholder} />

					<Dialog.Trigger asChild let:builder>
						<Button
							class="absolute bottom-2 right-2 translate-y-16 scale-0 opacity-0 transition-all duration-100 group-hover:translate-y-0 group-hover:scale-100 group-hover:opacity-100"
							builders={[builder]}
							size="icon"
							variant="outline"
						>
							<Tooltip.Root>
								<Tooltip.Trigger>
									<Eye class="size-4 text-muted-foreground" />
								</Tooltip.Trigger>
								<Tooltip.Content>
									<p>View generation</p>
								</Tooltip.Content>
							</Tooltip.Root></Button
						></Dialog.Trigger
					>
				</div>
			{/if}
		</div>
		<Dialog.Content
			class="flex h-full max-w-none flex-col items-center  justify-center overflow-y-scroll  md:h-fit md:w-fit md:flex-none"
		>
			<div class="w-fit scale-90 md:scale-100">
				<Dialog.Title>Generation 1</Dialog.Title>
				<Dialog.Description class="pb-4">
					{new Date()}
				</Dialog.Description>
				<slot />
			</div>
			<!-- <div class="grid w-full gap-4 py-4"> -->
			<!-- 	<div class="grid grid-cols-4 items-center gap-4"> -->
			<!-- 		<Label for="name" class="text-right">Name</Label> -->
			<!-- 		<Input id="name" value="Pedro Duarte" class="col-span-3" /> -->
			<!-- 	</div> -->
			<!-- 	<div class="grid grid-cols-4 items-center gap-4"> -->
			<!-- 		<Label for="username" class="text-right">Username</Label> -->
			<!-- 		<Input id="username" value="@peduarte" class="col-span-3" /> -->
			<!-- 	</div> -->
			<!-- </div> -->
			<!-- <Dialog.Footer> -->
			<!-- 	<Button type="submit">Save changes</Button> -->
			<!-- </Dialog.Footer> -->
		</Dialog.Content>
	</Dialog.Root>
{:else}
	<div class="group relative aspect-video h-fit w-full overflow-clip rounded-sm">
		<img
			in:blur={{ duration: 300 }}
			src="placeholder.svg"
			class="h-full w-full object-cover object-center grayscale transition-all duration-300 hover:grayscale-0"
			loading="lazy"
		/>
		<div
			class="absolute left-0 top-0 h-full w-full backdrop-blur-sm transition-all duration-100 ease-in group-hover:backdrop-blur-md"
		></div>

		<Button
			class="absolute bottom-2 right-2 translate-y-16 scale-0 opacity-0 transition-all duration-100 group-hover:translate-y-0 group-hover:scale-100 group-hover:opacity-100"
			on:click={() => {
				spin = true;
				setTimeout(() => {
					checkImage(img);
				}, 3000);
			}}
			size="icon"
			variant="outline"
		>
			<Tooltip.Root>
				<Tooltip.Trigger>
					<RefreshCcw
						class={spin
							? 'size-4 animate-spin text-muted-foreground'
							: 'size-4 text-muted-foreground'}
					/>
				</Tooltip.Trigger>
				<Tooltip.Content>
					<p>Refresh</p>
				</Tooltip.Content>
			</Tooltip.Root></Button
		>
	</div>
{/if}
