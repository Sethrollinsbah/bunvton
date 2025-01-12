<script lang="ts">
	import { blur } from 'svelte/transition';
	import { selectedImage } from '$lib';
	import Upload from 'lucide-svelte/icons/upload';
	import { onDestroy } from 'svelte';
	onDestroy(() => {
		$selectedImage = { e: false, link: '' };
	});

	function regenerateCall() {
		throw new Error('Function not implemented.');
	}
	import { onMount } from 'svelte';
	import * as Dialog from './ui/dialog';
	import Button from './ui/button/button.svelte';
	import * as Tooltip from './ui/tooltip';
	import { Eye, Fullscreen, RefreshCcw } from 'lucide-svelte';
	let placeholder: typeof import('./svg/Placeholder.svelte').default;

	onMount(async () => {
		// Dynamically import the SVG component
		const { default: PlaceholderSvg } = await import('./svg/Placeholder.svelte');
		placeholder = PlaceholderSvg;
		$selectedImage.link = result[0].link;
	});
	onDestroy(() => {
		$selectedImage = { link: '', e: false };
	});
	export let items; //id title link
	export let result;
</script>

<div class="flex h-fit w-fit flex-col space-y-2 md:h-96 md:flex-row md:space-x-2 md:space-y-0">
	<div
		class="aspect-square h-full w-full max-w-[30rem] rounded-md object-cover opacity-100 transition-all duration-100 group-hover:opacity-50 md:max-h-96 md:max-w-96"
	>
		{#if result && result[0].link}
			<div class="group relative aspect-square h-fit w-full">
				<!-- {#if exists} -->
				<div
					style="background-image: url({result[0].link});"
					class="object-start h-full w-full rounded-md object-contain opacity-100 transition-all duration-100 group-hover:object-contain group-hover:opacity-50 md:max-h-96 md:max-w-96"
				></div>
				<!-- {:else} -->
				<!-- <p>Image not found. Displaying fallback.</p> -->
				<!-- {/if} -->

				<Dialog.Trigger asChild let:builder>
					<Button
						on:click={() => {
							$selectedImage.e = false;
							$selectedImage.e = !$selectedImage.e;
							$selectedImage.link = items[2][2];
						}}
						class="absolute bottom-2 right-2 translate-y-16 scale-0 opacity-0 transition-all duration-100 group-hover:translate-y-0 group-hover:scale-100 group-hover:opacity-100"
						builders={[builder]}
						size="icon"
						variant="outline"
					>
						<Tooltip.Root>
							<Tooltip.Trigger>
								<Fullscreen class="size-4 text-muted-foreground" />
							</Tooltip.Trigger>
							<Tooltip.Content>
								<p>View generation</p>
							</Tooltip.Content>
						</Tooltip.Root></Button
					></Dialog.Trigger
				>
			</div>
		{:else if placeholder}
			<div class="group relative aspect-square h-fit w-full">
				<svelte:component
					this={placeholder}
					className="aspect-square  group-hover:opacity-50 opacity-100 transition-all duration-100 h-full w-full max-w-[30rem] rounded-md object-cover md:max-h-96 md:max-w-96"
				/>

				<Dialog.Trigger asChild let:builder>
					<Button
						on:click={() => {
							$selectedImage.e = false;
							$selectedImage.e = !$selectedImage.e;
						}}
						class="absolute bottom-2 right-2 translate-y-16 scale-0 opacity-0 transition-all duration-100 group-hover:translate-y-0 group-hover:scale-100 group-hover:opacity-100"
						builders={[builder]}
						size="icon"
						variant="outline"
					>
						<Tooltip.Root>
							<Tooltip.Trigger>
								<Fullscreen class="size-4 text-muted-foreground" />
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
	<div
		class="flex h-full max-h-32 w-full max-w-[30rem] grid-cols-3 flex-row justify-between space-x-2 sm:w-fit md:flex md:max-h-96 md:w-32 md:flex-col md:gap-2 md:space-x-0"
	>
		{#each items as i, ind}
			<div
				class="flex aspect-square h-full w-full items-center justify-center rounded-md border border-dashed md:h-32 md:w-32"
			>
				<div
					class="aspect-square h-full w-full max-w-[30rem] rounded-md object-cover opacity-100 transition-all duration-100 group-hover:opacity-50 md:max-h-96 md:max-w-96"
				>
					{#if i[2] && i[2] !== null}
						<div class="group relative aspect-square h-fit w-full">
							<img
								in:blur={{ duration: 300 }}
								alt="model"
								src={i[2]}
								class="aspect-square h-full w-full rounded-md object-cover object-center opacity-100 transition-all duration-100 group-hover:opacity-50"
								loading="lazy"
							/>
							<Dialog.Trigger asChild let:builder>
								<Button
									on:click={() => {
										$selectedImage.e = false;
										$selectedImage.e = !$selectedImage.e;
										$selectedImage.link = i[ind][2];
									}}
									class="absolute bottom-2 right-2 translate-y-16 scale-0 opacity-0 transition-all duration-100 group-hover:translate-y-0 group-hover:scale-100 group-hover:opacity-100"
									builders={[builder]}
									size="icon"
									variant="outline"
								>
									<Tooltip.Root>
										<Tooltip.Trigger>
											<Fullscreen class="size-4 text-muted-foreground" />
										</Tooltip.Trigger>
										<Tooltip.Content>
											<p>View {i[1]}</p>
										</Tooltip.Content>
									</Tooltip.Root></Button
								></Dialog.Trigger
							>
						</div>
					{:else if placeholder}
						<div class="group relative aspect-square h-fit w-full">
							<svelte:component
								this={placeholder}
								svgsize="160"
								className="aspect-square group-hover:opacity-50 opacity-100 transition-all duration-100  h-full w-full rounded-md object-cover"
							/>

							<Dialog.Trigger asChild let:builder>
								<Button
									on:click={() => {
										$selectedImage.e = false;
										$selectedImage.e = !$selectedImage.e;
										$selectedImage.link = i[ind][2];
									}}
									class="absolute bottom-2 right-2 translate-y-16 scale-0 opacity-0 transition-all duration-100 group-hover:translate-y-0 group-hover:scale-100 group-hover:opacity-100"
									builders={[builder]}
									size="icon"
									variant="outline"
								>
									<Tooltip.Root>
										<Tooltip.Trigger>
											<Fullscreen class="size-4 text-muted-foreground" />
										</Tooltip.Trigger>
										<Tooltip.Content>
											<p>View {i[1]}</p>
										</Tooltip.Content>
									</Tooltip.Root></Button
								></Dialog.Trigger
							>
						</div>
					{/if}
				</div>
			</div>
		{/each}

		<Button
			on:click={async () => {
				await regenerateCall();
			}}
			variant={'ghost'}
			class="flex aspect-square h-full w-full items-center justify-center rounded-md border border-dashed
			md:h-32 md:w-32"
		>
			<Tooltip.Root>
				<Tooltip.Trigger>
					<div class="animate-spin">
						<RefreshCcw class="size-4 text-muted-foreground" />
					</div>
				</Tooltip.Trigger>
				<Tooltip.Content>
					<p>Rerun model</p>
				</Tooltip.Content>
			</Tooltip.Root>
		</Button>
	</div>
</div>
<div class="mt-4 h-[1px] w-full bg-muted"></div>
