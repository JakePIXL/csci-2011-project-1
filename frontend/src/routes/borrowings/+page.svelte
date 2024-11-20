<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import type { BorrowedBook } from '$lib/views';
	import { onMount } from 'svelte';

	let borrowings: BorrowedBook[] = [];
	let loading = true;
	let error: string | null = null;

	onMount(async () => {
		try {
			const response = await fetch('/api/borrows/?limit=300');
			borrowings = await response.json();
		} catch (e) {
			error = 'Failed to load borrowings';
		} finally {
			loading = false;
		}
	});

	async function returnBook(id: Number) {
		try {
			const response = await fetch(`/api/borrows/return/`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ book_id: id })
			});

			if (response.ok) {
				invalidateAll();
			} else {
				console.log('Failed to return book');
			}
		} catch {
			console.log('Failed to return book');
		}
	}
</script>

<div class="container mx-auto px-4 py-8">
	<div class="mb-6 flex items-center justify-between">
		<div class="flex flex-row items-center gap-4">
			<button
				onclick={() => window.history.back()}
				class="rounded-md bg-slate-200 px-2 py-1 text-xs hover:bg-slate-400">back</button
			>
			<h1 class="text-2xl font-bold">Borrowings</h1>
		</div>
		<!-- <a href="/borrowings/new" class="rounded-md bg-blue-500 px-4 py-2 text-white hover:bg-blue-600">
			New Borrowing
		</a> -->
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
							Book
						</th>
						<th
							class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
						>
							Borrower
						</th>
						<th
							class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
						>
							Borrow Date
						</th>
						<th
							class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
						>
							Return Date
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
					{#each borrowings as borrowing}
						<tr>
							<td class="whitespace-nowrap px-6 py-4">
								{borrowing.title} by {borrowing.author}
							</td>
							<td class="whitespace-nowrap px-6 py-4">{borrowing.borrower}</td>
							<td class="whitespace-nowrap px-6 py-4">{borrowing.borrow_date}</td>
							<td class="whitespace-nowrap px-6 py-4">{borrowing.return_date || '-'}</td>
							<td class="whitespace-nowrap px-6 py-4">
								<span
									class="rounded-full px-2 py-1 text-xs font-semibold"
									class:bg-yellow-100={!borrowing.return_date}
									class:text-yellow-800={!borrowing.return_date}
									class:bg-green-100={borrowing.return_date}
									class:text-green-800={borrowing.return_date}
								>
									{borrowing.return_date ? 'returned' : 'borrowed'}
								</span>
							</td>
							<td class="whitespace-nowrap px-6 py-4">
								{#if !borrowing.return_date}
									<button
										class="text-blue-600 hover:text-blue-900"
										onclick={() => returnBook(borrowing.id)}
									>
										Return
									</button>
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
