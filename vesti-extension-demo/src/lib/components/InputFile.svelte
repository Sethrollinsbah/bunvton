<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { writable } from 'svelte/store';
	import { Download, Eye, Fullscreen, Loader, Trash } from 'lucide-svelte';
	import Button from './ui/button/button.svelte';
	import * as Tooltip from './ui/tooltip';

	let loading = false;
	let files: File[] = [];

	const uploadStatus = writable(0);

	async function onChange(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		if (processing) return;
		loading = true;

		const input = event.currentTarget;
		const selectedFiles = input.files ?? [];
		const validFiles: File[] = [];

		// Process each file
		for (const file of selectedFiles) {
			if (file.type.startsWith('image/')) {
				validFiles.push(file);
			} else {
				console.error(`Invalid file type: ${file.name}`);
				toast.error(`Unsupported file type: ${file.name}`);
			}
		}

		// Append valid files to the array
		files = [...files, ...validFiles];
		console.log('Selected files:', files);

		loading = false;
	}

	export let typeOfFile = 'image'; // Adjust type for images
	export let processing = false;
	let processedFile = writable(0);

	let progress: HTMLElement;

	let filesTotal = 0;
	async function handlePutRequest() {
		processing = true;

		filesTotal = files.length;
		const filePromises = files.map(async (file) => {
			try {
				const formData = new FormData();
				formData.append('file', file);

				const uploadResponse = await fetch(`/api/files/${typeOfFile}`, {
					method: 'POST',
					body: formData
				});

				if (uploadResponse.ok) {
					console.log(`File ${file.name} uploaded successfully`);
					uploadStatus.set(1);
				} else {
					console.error(`Upload failed: ${uploadResponse.statusText}`);
					uploadStatus.set(4);
				}
			} catch (error) {
				console.error('Error:', error);
				uploadStatus.set(4);
			} finally {
				files = files.filter((f) => f.name !== file.name); // Remove processed file
				$processedFile += 1;
				updateProgressBar();
			}
		});

		try {
			await Promise.all(filePromises);
		} finally {
			processing = false;
			$processedFile = 0;
			filesTotal = 0;
		}
	}

	function updateProgressBar() {
		const progressPercentage = ($processedFile / filesTotal) * 100;
		progress.style.width = `${progressPercentage}%`;
	}

	function removeFile(index: number) {
		files = files.filter((_, i) => i !== index);
	}
	export let chosenFile;
	$: chosenFile = files;
</script>

{#each files as f, index}
	<!-- {#if index === 0} -->
	<div
		class="group relative mt-2 flex h-40 w-full min-w-40 cursor-pointer flex-col items-center justify-center rounded-md border border-zinc-300 bg-white bg-cover bg-center shadow-sm transition-all duration-300 hover:bg-zinc-50 hover:bg-contain dark:border-zinc-700 dark:bg-zinc-950"
		style="background-image: url({URL.createObjectURL(new Blob([f]))});"
	>
		<!-- <img -->
		<!-- 	src={} -->
		<!-- 	alt="garment" -->
		<!-- 	class="absolute left-0 top-0 h-full w-full" -->
		<!-- /> -->

		<Button
			on:click={() => removeFile(index)}
			class="absolute bottom-2 right-2 z-10 translate-y-16 scale-0 opacity-0 transition-all duration-100 group-hover:translate-y-0 group-hover:scale-100 group-hover:opacity-100"
			size="icon"
			variant="outline"
		>
			<Tooltip.Root>
				<Tooltip.Trigger>
					<Trash class="size-4 text-destructive" />
				</Tooltip.Trigger>
				<Tooltip.Content>
					<p>Delete upload</p>
				</Tooltip.Content>
			</Tooltip.Root></Button
		>
	</div>
	<!-- {/if} -->
{/each}
{#if files.length === 0}
	<form
		on:submit={async () => {
			toast.promise(handlePutRequest(), {
				success: 'Done',
				error: 'Error',
				loading: 'Loading`'
			});
		}}
		class="relative mt-2 grid h-40 w-full text-sm"
	>
		<div>
			<label
				for="image-upload"
				class="group relative flex h-full cursor-pointer flex-col items-center justify-center rounded-md border border-zinc-300 bg-white shadow-sm transition-all duration-300 hover:bg-zinc-50 dark:border-zinc-700 dark:bg-zinc-950"
			>
				{#if files.length === 0}
					<div class="relative z-[5] h-full w-full rounded-md">
						<div
							class="relative z-[3] flex h-full w-full flex-col items-center justify-center rounded-md bg-white px-10 opacity-100 transition-all duration-300 hover:bg-zinc-50 dark:bg-zinc-950"
						>
							{#if loading}
								<Loader class="animate-spin" />
							{:else}
								<slot />
							{/if}
							<p class="mt-2 text-center text-sm text-zinc-500">Click to upload.</p>
							<p class="mt-2 text-center text-sm text-zinc-500">Max file size: 5MB</p>
							<span class="sr-only">Photo upload</span>
						</div>
					</div>
				{:else}
					<div class="h-full w-full overflow-y-scroll">
						{#each files as f, index}
							<div class="flex h-fit w-full flex-col items-center justify-start"></div>
						{/each}
					</div>
				{/if}
			</label>
			<div class="mt-1 flex rounded-md shadow-sm">
				<input
					id="image-upload"
					disabled={processing}
					name="image-upload"
					type="file"
					accept="image/jpeg"
					class="sr-only"
					on:change={(e) => {
						toast.promise(onChange(e), {
							success: 'Success',
							loading: 'Loading...',
							error: 'Error'
						});
					}}
					on:drop={(e) => {
						toast.promise(onChange(e), {
							success: 'Success',
							loading: 'Loading...',
							error: 'Error'
						});
					}}
					on:dragover={(ev) => ev.preventDefault()}
				/>
			</div>
		</div>
	</form>
{/if}
