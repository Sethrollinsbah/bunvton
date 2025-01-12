<script lang="ts">
	import { modal } from '$lib';
	import { Button, buttonVariants } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Separator } from './ui/select';
	import Textarea from './ui/textarea/textarea.svelte';
	function handleChange(e) {
		$modal = $modal === 'feedback' ? '' : 'feedback';
	}
	import * as Select from '$lib/components/ui/select';
	let topics = ['VTON Generation'];
</script>

<Dialog.Root
	open={$modal === 'feedback'}
	onOutsideClick={() => {
		$modal = '';
	}}
	onOpenChange={handleChange}
>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Feedback</Dialog.Title>
			<div class="grid w-full gap-2">
				<Label for="msg" class="mt-2 text-left">Message</Label>

				<Textarea disabled={true} id="msg" class="h-32" placeholder="Type your message here." />

				<Button class="mt-4">Send message</Button>
			</div>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
