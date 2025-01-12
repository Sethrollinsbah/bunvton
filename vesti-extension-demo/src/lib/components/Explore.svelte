<script>
	import { obj } from '$lib';
</script>

<section class="grid h-full w-full">
	{#if $obj.length > 0}
		<div
			class="mx-auto grid h-fit w-full max-w-md grid-cols-1 gap-4 px-4 sm:max-w-md md:max-w-4xl md:grid-cols-2 lg:max-w-5xl lg:grid-cols-3 xl:max-w-6xl"
		>
			<h1
				class:from-[hsl(var(--success))]={false}
				class="z-20 col-span-full mt-4 bg-gradient-to-r from-muted-foreground via-black to-black/50 bg-clip-text text-4xl font-medium text-transparent"
			>
				History
			</h1>
			<slot />
		</div>
	{/if}

	<div
		class="mx-auto mt-16 flex h-fit w-screen max-w-md flex-col text-center sm:max-w-md md:max-w-2xl lg:max-w-3xl xl:max-w-4xl"
	>
		<h1 class="mb-2 text-2xl font-bold sm:text-3xl lg:text-4xl">
			Virtual Tryons in just <span class="text-blue-800">one click</span>
		</h1>
		<p class="mb-8 text-base text-primary/80 lg:text-lg">
			Follow the demo to see how to create your own generation
		</p>
		<div
			class="z-10 flex w-full flex-col items-center justify-center overflow-clip rounded-3xl bg-[url('/blobs.jpeg')] bg-cover bg-center shadow-xl"
		>
			<div class="z-20 h-full w-full bg-background/90 px-16 py-8 backdrop-blur-2xl">
				<div class="aspect-video h-fit w-full p-0">
					<video class="rounded-md" autoplay>
						<source
							src="https://pub-82f74f0ef5a241518dfdaec5fa1292fa.r2.dev/demo.mov"
							type="video/quicktime"
						/>
						Your browser does not support the video tag.
					</video>
				</div>
			</div>
		</div>
	</div>
</section>
