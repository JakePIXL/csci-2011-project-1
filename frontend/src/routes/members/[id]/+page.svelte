<script lang="ts">
	import type { PageData } from './$types';
	import { invalidateAll, goto } from '$app/navigation';

	let { data }: { data: PageData } = $props();

	let editedMember = {
		first_name: data.member.first_name,
		last_name: data.member.last_name,
		email: data.member.email,
		phone: data.member.phone || ''
	};

	async function updateMember() {
		try {
			const response = await fetch(`/api/members/${data.member.id}`, {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(editedMember)
			});

			if (response.ok) {
				const updatedMember = await response.json();
				data.member = updatedMember;

				goto('/members');
			} else {
				console.log('Failed to update member');
			}
		} catch {
			console.log('Failed to update member');
		}
	}

	async function deleteMember() {
		try {
			const response = await fetch(`/api/members/${data.member.id}`, {
				method: 'DELETE'
			});

			if (response.ok) {
				goto('/members');
			} else {
				console.log('Failed to delete member');
			}
		} catch {
			console.log('Failed to delete member');
		}
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
					<form onsubmit={updateMember} class="mt-4 space-y-4">
						<div>
							<label class="text-sm font-medium text-gray-600" for="first_name">First Name</label>
							<input
								type="text"
								id="first_name"
								bind:value={editedMember.first_name}
								class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-blue-500"
							/>
						</div>
						<div>
							<label class="text-sm font-medium text-gray-600" for="last_name">Last Name</label>
							<input
								type="text"
								id="last_name"
								bind:value={editedMember.last_name}
								class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-blue-500"
							/>
						</div>
						<div>
							<label class="text-sm font-medium text-gray-600" for="email">Email</label>
							<input
								type="email"
								id="email"
								bind:value={editedMember.email}
								class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-blue-500"
							/>
						</div>
						<div>
							<label class="text-sm font-medium text-gray-600" for="phone">Phone</label>
							<input
								type="tel"
								id="phone"
								bind:value={editedMember.phone}
								class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-blue-500"
							/>
						</div>
						<button
							type="submit"
							class="mt-4 rounded-md bg-blue-500 px-4 py-2 text-white hover:bg-blue-600"
						>
							Update Information
						</button>
					</form>
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
