<script lang="ts">
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	let isDarkMode = false;

	// Define properties for the sun and moon transitions
	const properties = {
		sun: {
			r: 9,
			transform: 'rotate(40deg)',
			cx: 12,
			cy: 4,
			opacity: 1
		},
		moon: {
			r: 5,
			transform: 'rotate(90deg)',
			cx: 30,
			cy: 0,
			opacity: 0
		},
		springConfig: { duration: 500, easing: cubicOut }
	};

	// Tweened values for smooth transitions
	let r = tweened(properties.sun.r, properties.springConfig);
	let transform = tweened(properties.sun.transform, properties.springConfig);
	let cx = tweened(properties.sun.cx, properties.springConfig);
	let cy = tweened(properties.sun.cy, properties.springConfig);
	let opacity = tweened(properties.sun.opacity, properties.springConfig);

	// Function to toggle between sun and moon
	function toggleMode() {
		const target = isDarkMode ? properties.sun : properties.moon;
		isDarkMode = !isDarkMode;
		r.set(target.r);
		transform.set(target.transform);
		cx.set(target.cx);
		cy.set(target.cy);
		opacity.set(target.opacity);
	}
</script>

<div>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		width="24"
		height="24"
		viewBox="0 0 24 24"
		fill="none"
		stroke="currentColor"
		stroke-width="2"
		stroke-linecap="round"
		stroke-linejoin="round"
		on:click={toggleMode}
		style="transform: {$transform};"
	>
		<mask id="mask">
			<rect x="0" y="0" width="100%" height="100%" fill="white" />
			<circle cx="12" cy="4" r={$r} fill="black" style="cx: {$cx}; cy: {$cy};" />
		</mask>
		<circle fill="black" cx="12" cy="12" r="9" mask="url(#mask)" />

		<g style="opacity: {$opacity};" fill="currentColor">
			<line x1="12" y1="1" x2="12" y2="3" />
			<line x1="12" y1="21" x2="12" y2="23" />
			<line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
			<line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
			<line x1="1" y1="12" x2="3" y2="12" />
			<line x1="21" y1="12" x2="23" y2="12" />
			<line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
			<line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
		</g>
	</svg>
</div>

<style>
	svg {
		cursor: pointer;
	}
</style>
