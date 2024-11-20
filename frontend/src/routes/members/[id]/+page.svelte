<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	function deleteMember() {
		fetch(`/api/members/${data.member.id}`, {
			method: 'DELETE'
		}).then(() => {
			window.location.href = '/';
		});
	}
</script>

<div class="container mx-auto bg-white px-6 py-8">
	<div class="flex flex-col space-y-6">
		<div class="flex items-center justify-between">
			<div class="flex flex-row items-center gap-4">
				<button
					onclick={() => window.history.back()}
					class="rounded-md bg-slate-200 px-2 py-1 text-xs hover:bg-slate-400">back</button
				>
				<h1 class="text-2xl font-bold">Members</h1>
			</div>
			<div class="flex flex-row items-center gap-4">
				<a
					href={`/members/${data.member.id}/new`}
					class="rounded-md bg-green-500 px-4 py-2 text-white hover:bg-green-600"
				>
					New Checkout
				</a>
				<button
					onclick={() => deleteMember()}
					class="rounded-md bg-red-500 px-4 py-2 text-white hover:bg-red-600"
				>
					Delete Member
				</button>
			</div>
		</div>

		<div class="rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
			<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
				<div>
					<h2 class="text-xl font-semibold text-gray-800">Personal Information</h2>
					<div class="mt-4 space-y-4">
						<div>
							<label class="text-sm font-medium text-gray-600">Name</label>
							<p class="text-gray-800">{data.member.name}</p>
						</div>
						<div>
							<label class="text-sm font-medium text-gray-600">Email</label>
							<p class="text-gray-800">{data.member.email}</p>
						</div>
						<div>
							<label class="text-sm font-medium text-gray-600">Phone</label>
							<p class="text-gray-800">{data.member.phone || 'Not provided'}</p>
						</div>
					</div>
				</div>
			</div>
		</div>

		<div class="rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
			<h2 class="text-xl font-semibold text-gray-800">Borrowing History</h2>
			<div class="mt-4 overflow-x-auto">
				<table class="min-w-full divide-y divide-gray-200">
					<thead class="bg-gray-50">
						<tr>
							<th
								class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
							>
								Book Title
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
							>
								Author
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
						</tr>
					</thead>
					<tbody class="divide-y divide-gray-200 bg-white">
						{#each data.borrowings as book}
							<tr>
								<td class="whitespace-nowrap px-6 py-4">"{book.title}"</td>
								<td class="whitespace-nowrap px-6 py-4">{book.author}</td>
								<td class="whitespace-nowrap px-6 py-4">
									{new Date(book.borrow_date).toLocaleDateString()}
								</td>
								<td class="whitespace-nowrap px-6 py-4">
									{book.return_date
										? new Date(book.return_date).toLocaleDateString()
										: 'Not returned'}
								</td>
								<td class="whitespace-nowrap px-6 py-4">
									<span
										class="inline-flex rounded-full px-2 text-xs font-semibold leading-5"
										class:bg-yellow-100={book.status === 'borrowed'}
										class:text-yellow-800={book.status === 'borrowed'}
										class:bg-green-100={book.status === 'available'}
										class:text-green-800={book.status === 'available'}
									>
										{book.status === 'available' ? 'returned' : 'borrowed'}
									</span>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	</div>
</div>
