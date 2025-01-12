// place iles you want to import through the `$lib` alias in this folder.
import { writable } from 'svelte/store';
export let selectedImage = writable<{ id: string; e: boolean; link: string }>({
	id: '',
	e: false,
	link: ''
});
export let isNightMode = writable<boolean>(false);
export let modal = writable<string>('');
export let ws = writable<string>('');
export let open = writable<Number>(2);
export let obj = writable<VtonGeneration[]>([
	// {
	// 	result: [{ link: 'https://pub-df74471e49be4a1ea65eaeed024865e4.r2.dev/vtona762154a-32ee-4018-b935-e9cab3070946___.png' }],
	// 	items: [
	// 		['1', 'model', '/kamala.jpeg'],
	// 		['2', 'garment', '/soreal.jpg']
	// 	]
	// }
]
);
type VtonGeneration = {
	result: { link: string }[];
	items: string[][];
};

// {
// 	result: [{ link: '/c02d850efff541d6ba32d595cad587a8.png' }],
// 	items: [
// 		['1', 'model', '/sommerRayFrom2016.png'],
// 		['2', 'garment', '/bikini.jpeg']
// 	]
// },

