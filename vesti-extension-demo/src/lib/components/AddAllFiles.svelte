<script lang="ts">
	import { Shirt, UserRound } from 'lucide-svelte';
	import InputFile from './InputFile.svelte';
	import { Label } from './ui/label';
	import { Button } from './ui/button';

	import { obj, open } from '$lib';
	import { browser } from '$app/environment';
	let garmentFile: File[] = [];
	let modelFile: File[] = [];
	let filesTotal = 0;
	let processing = false;
	let progress: HTMLElement;
	export let uploading: boolean;
	async function handlePutRequest(filesA: File[], filesB: File[]) {
		const vtonId = 'vton' + crypto.randomUUID();

		if (browser) {
			const personal = JSON.parse(localStorage.getItem('personal') || '[]') || [];

			localStorage.setItem('personal', JSON.stringify([...personal, vtonId]));
		}
		// Total number of files to process
		filesTotal = filesA.length + filesB.length;

		// Track processed files
		let garmentProcessedFile = 0;
		let modelProcessedFile = 0;
		processing = true;

		// Create FormData to send all the files in one request
		const formData = new FormData();

		// Append each file from filesA (models) to the FormData
		filesA.forEach((file: File) => formData.append('model', file));

		// Append each file from filesB (garments) to the FormData
		filesB.forEach((file: File) => formData.append('garment', file));

		garmentFile = [];
		modelFile = [];
		try {
			// Send the formData to your API
			const uploadResponse = await fetch(`/api/files?vtonId=` + vtonId, {
				method: 'PUT',
				body: formData // Send as FormData
			});

			if (uploadResponse.ok) {
				console.log('Files uploaded successfully');

				// Simulate processing of individual files for progress tracking
				const allFiles = [...filesA, ...filesB];

				for (const file of allFiles) {
					// Determine the type of file for progress tracking
					if (filesB.includes(file)) {
						garmentProcessedFile += 1;
					} else {
						modelProcessedFile += 1;
					}

					// Update progress bar for each file after "upload"
					updateProgressBar(garmentProcessedFile, modelProcessedFile, filesTotal);
				}
				$obj = [
					{
						result: [{ link: 'https://output.getvesti.com/' + vtonId + '___.png' }],
						items: [
							['1', 'model', 'https://input.getvesti.com/' + vtonId + '___model.jpg'],
							['2', 'garment', 'https://input.getvesti.com/' + vtonId + '___garment.jpg']
						]
					},
					...$obj
				];
			} else {
				console.error('Upload failed', uploadResponse.statusText);
			}
		} catch (error) {
			console.error('Error uploading files:', error);
		} finally {
			// Reset state after all uploads are done
			processing = false;
			garmentFile = [];
			modelFile = [];
			garmentProcessedFile = 0;
			modelProcessedFile = 0;
			$open = 2;
			uploading = true;
		}
	}

	function updateProgressBar(garmentProcessed, modelProcessed, totalFiles) {
		const processed = garmentProcessed + modelProcessed;
		const progressPercentage = (processed / totalFiles) * 100;
		if (progress) {
			progress.style.width = `${progressPercentage}%`;
		}
	}

	function handleUpload() {
		if (garmentFile.length > 0 && modelFile.length > 0) {
			processing = true;
			handlePutRequest(garmentFile, modelFile).then(() => {
				$open = 2;
				garmentFile = [];
				modelFile = [];
			});
		}
	}
</script>

<!-- File Input Fields for Garment and Model -->

<div class="grid h-full grid-cols-2 justify-between space-x-2 py-4">
	<div>
		<Label>Garment</Label>
		<InputFile bind:chosenFile={modelFile} typeOfFile="model">
			<Shirt
				class="h-7 w-7 scale-100 text-zinc-500 transition-all duration-75 group-hover:scale-110 group-active:scale-95"
			/>
		</InputFile>
	</div>
	<div>
		<Label>Model</Label>
		<InputFile bind:chosenFile={garmentFile} typeOfFile="garment">
			<UserRound
				class="h-7 w-7 scale-100 text-zinc-500 transition-all duration-75 group-hover:scale-110 group-active:scale-95"
			/>
		</InputFile>
	</div>
</div>

<!-- Progress Bar and Upload Button -->
{#if processing}
	<div class="relative mb-4 h-2.5 w-full rounded-full bg-gray-200 dark:bg-gray-700">
		<div
			bind:this={progress}
			class="absolute h-2.5 rounded-full bg-green-600 transition-all duration-200 dark:bg-green-300"
		></div>
	</div>
	<p>Don't close this page</p>
{:else}
	<Button
		class="mx-auto mt-4"
		type="submit"
		on:click={handleUpload}
		disabled={garmentFile.length === 0 || modelFile.length === 0}
	>
		Upload
	</Button>
{/if}
