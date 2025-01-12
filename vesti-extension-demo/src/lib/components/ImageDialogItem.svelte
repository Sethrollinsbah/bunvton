<script lang="ts">
	import * as Tooltip from './ui/tooltip';
	import { Button, buttonVariants } from './ui/button';
	import * as Dialog from './ui/dialog';
	import { selectedImage } from '$lib';
	import { onDestroy, onMount } from 'svelte';
	let placeholder: typeof import('./svg/Placeholder.svelte').default;

	onMount(async () => {
		// Dynamically import the SVG component
		const { default: PlaceholderSvg } = await import('./svg/Placeholder.svelte');
		placeholder = PlaceholderSvg;
	});
</script>

<Dialog.Root open={$selectedImage.e} onOpenChange={() => $selectedImage.e}>
	<Dialog.Content class="h-full w-full py-8 md:h-fit md:max-w-none">
		<div class="h-40 w-full">
			<Dialog.Title>Generation 1</Dialog.Title>
			<Dialog.Description>
				{new Date()}
			</Dialog.Description>
			{#if $selectedImage.link}
				<div class="group relative aspect-square h-fit w-full">
					<img
						src={$selectedImage.link}
						alt="result"
						class="aspect-square h-full w-full rounded-md object-cover opacity-100 transition-all duration-100 group-hover:opacity-50 md:max-h-96 md:max-w-96"
					/>
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
	</Dialog.Content>
</Dialog.Root>
