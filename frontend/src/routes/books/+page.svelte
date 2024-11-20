<script lang="ts">
	import type { Book } from '$lib/views';
	import { Status } from '$lib/views';
	import { onMount } from 'svelte';

	let books: Book[] = [];
	let loading = true;
	let error: string | null = null;

	onMount(async () => {
		try {
			const response = await fetch('/api/books/?limit=300');
			books = await response.json();
		} catch (e) {
			error = 'Failed to load books';
		} finally {
			loading = false;
		}
	});
</script>

<div class="container mx-auto px-4 py-8">
	<div class="mb-6 flex items-center justify-between">
		<div class="flex flex-row items-center gap-4">
			<button
				onclick={() => window.history.back()}
				class="rounded-md bg-slate-200 px-2 py-1 text-xs hover:bg-slate-400">back</button
			>
			<h1 class="text-2xl font-bold">Books</h1>
		</div>
		<a href="/books/new" class="rounded-md bg-blue-500 px-4 py-2 text-white hover:bg-blue-600">
			Add New Book
		</a>
	</div>

	{#if loading}
		<div class="text-center">Loading...</div>
	{:else if error}
		<div class="text-red-500">{error}</div>
	{:else}
		<div class="overflow-x-auto">
			<table class="min-w-full divide-y divide-gray-200">
				<thead class="bg-gray-50">
					<tr>
						<th
							class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
						>
							Title
						</th>
						<th
							class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
						>
							Author
						</th>
						<th
							class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
						>
							Category
						</th>
						<th
							class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
						>
							Status
						</th>
						<th
							class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
						>
							Actions
						</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-gray-200 bg-white">
					{#each books as book}
						<tr>
							<td class="whitespace-nowrap px-6 py-4">"{book.title}"</td>
							<td class="whitespace-nowrap px-6 py-4">{book.author}</td>
							<td class="whitespace-nowrap px-6 py-4">{book.category || '-'}</td>
							<td class="whitespace-nowrap px-6 py-4">
								<span
									class="rounded-full px-2 py-1 text-xs font-semibold"
									class:bg-green-100={book.status === Status.Available}
									class:text-green-800={book.status === Status.Available}
									class:bg-red-100={book.status === Status.Borrowed}
									class:text-red-800={book.status === Status.Borrowed}
								>
									{book.status}
								</span>
							</td>
							<td class="whitespace-nowrap px-6 py-4">
								<a href="/books/{book.id}" class="text-blue-600 hover:text-blue-900">Edit</a>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
