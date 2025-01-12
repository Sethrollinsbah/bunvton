<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import TextBox from '$lib/components/TextBox.svelte';
	import ProductImages from '$lib/components/ProductImages.svelte';
	import ImageDialog from '$lib/components/ImageDialog.svelte';
	import Explore from '$lib/components/Explore.svelte';
	import { Button } from '$lib/components/ui/button';
	import { onMount } from 'svelte';
	import { obj } from '$lib';
	let personal = [];
	onMount(async () => {
		const filees = await fetch('/api/files');
		if (browser) {
			personal = JSON.parse(localStorage.getItem('personal') || '[]');
			localStorage.setItem('personal', JSON.stringify(personal));
		}
		for (let index = 0; index < personal.length; index++) {
			const element = personal[index];

			$obj = [
				{
					result: [{ link: 'https://output.getvesti.com/' + element + '___.png' }],
					items: [
						['1', 'model', 'https://input.getvesti.com/' + element + '___model.jpg'],
						['2', 'garment', 'https://input.getvesti.com/' + element + '___garment.jpg']
					]
				},
				...$obj
			];
		}
	});
	import { browser } from '$app/environment';
	async function getImage(url: string) {
		const image = await fetch(url);
		console.log(url);
		if (image.ok) {
			console.log(image.status);
			if (image.status !== 200) {
				console.log('error');
			}
		}
	}
</script>

<TextBox></TextBox>

<Explore>
	{#each $obj as o}
		<div class="relative">
			<ImageDialog img={o.result[0].link}>
				<ProductImages result={o.result} items={o.items}></ProductImages>
			</ImageDialog>
			<div class="absolute left-2 top-1">
				{#if o.result[0].tags}
					{#each o.result[0].tags as t}
						<Badge
							class="dakr:bg-zinc-700 my-2 mr-1 rounded-full bg-zinc-300 text-zinc-700 dark:bg-zinc-400"
							>#{t}</Badge
						>
					{/each}
				{/if}
			</div>
		</div>
		<!-- {/await} -->
	{/each}
	<!-- <Button variant="ghost">Close</Button> -->
</Explore>
