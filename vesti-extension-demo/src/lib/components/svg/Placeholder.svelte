<script>
	import { blur } from 'svelte/transition';
	let generating = false;
	export let className =
		'absolute left-1/2 top-1/2 z-0 h-full w-full -translate-x-1/2 -translate-y-1/2 rounded-md object-cover opacity-100 transition-all duration-100 group-hover:opacity-50';
	export let svgsize = '100';
	export let svgheight = svgsize;
	export let svgwidth = svgsize;
	export let img = '/placeholder.svg';
</script>

<img
	in:blur={{ duration: 500 }}
	alt="Product"
	class:animate-pulse={generating}
	class={className}
	height={svgheight}
	src={img}
	width={svgwidth}
/>
