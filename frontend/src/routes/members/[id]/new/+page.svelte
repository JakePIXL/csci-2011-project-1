<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';

	let { data }: { data: PageData } = $props();
	let selectedBook: any = $state(null);
	let selectedBookDetails: string = $state('');
	// Handle book selection
	function handleBookSelect(event: Event) {
		const select = event.target as HTMLSelectElement;
		selectedBook = data.books.find((book) => book.id === parseInt(select.value));
		selectedBookDetails = selectedBook ? JSON.stringify(selectedBook, null, 2) : '';
	}

	// Handle form submission
	async function handleSubmit(event: Event) {
		event.preventDefault();

		if (!selectedBook) return;

		try {
			const response = await fetch(`/api/borrows/${data.member.id}`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					book_id: selectedBook.id
				})
			});

			if (response.ok) {
				goto(`/members/${data.member.id}`);
			} else {
				alert('Failed to checkout book');
			}
		} catch (error) {
			console.error('Error:', error);
			alert('Failed to checkout book');
		}
	}
</script>

<div class="container mx-auto gap-4 bg-white px-6 py-8">
	<div class="flex flex-row items-center gap-4">
		<button
			onclick={() => window.history.back()}
			class="rounded-md bg-slate-200 px-2 py-1 text-xs hover:bg-slate-400">back</button
		>
		<h1 class="text-2xl font-bold">
			New Checkout for "{data.member.first_name + ' ' + data.member.last_name}"
		</h1>
	</div>
	<div class="rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
		<form onsubmit={handleSubmit} class="space-y-4">
			<div>
				<label for="book" class="block text-sm font-medium text-gray-700"> Select Book </label>
				<select
					id="book"
					class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
					onchange={handleBookSelect}
				>
					<option value="">Select a book</option>
					{#each data.books as book}
						{#if book.status === 'available'}
							<option value={book.id}>
								{book.title} by {book.author}
							</option>
						{/if}
					{/each}
				</select>
			</div>

			<div>
				<label for="details" class="block text-sm font-medium text-gray-700"> Book Details </label>
				<div class="mt-1 max-h-48 overflow-y-auto rounded-md border border-gray-300 bg-gray-50 p-4">
					{#if selectedBook}
						<p class="text-sm text-gray-900">
							<span class="font-medium">Title:</span>
							{selectedBook.title}
						</p>
						<p class="text-sm text-gray-900">
							<span class="font-medium">Author:</span>
							{selectedBook.author}
						</p>
						<p class="text-sm text-gray-900">
							<span class="font-medium">Category:</span>
							{selectedBook.category}
						</p>
						<p class="text-sm text-gray-900">
							<span class="font-medium">Status:</span>
							<span
								class={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${
									selectedBook.status === 'available'
										? 'bg-green-100 text-green-800'
										: 'bg-red-100 text-red-800'
								}`}>{selectedBook.status}</span
							>
						</p>
					{:else}
						<p class="text-sm italic text-gray-500">Select a book to view details</p>
					{/if}
				</div>
			</div>

			<div>
				<button
					type="submit"
					class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
					disabled={!selectedBook}
				>
					Checkout Book
				</button>
			</div>
		</form>
	</div>
</div>
