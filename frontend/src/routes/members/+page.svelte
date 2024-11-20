<script lang="ts">
	import type { Member } from '$lib/views';
	import { onMount } from 'svelte';

	let members: Member[] = [];
	let loading = true;
	let error: string | null = null;

	onMount(async () => {
		try {
			const response = await fetch('/api/members/');
			members = await response.json();
		} catch (e) {
			error = 'Failed to load members';
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
			<h1 class="text-2xl font-bold">Members</h1>
		</div>
		<a href="/members/new" class="rounded-md bg-blue-500 px-4 py-2 text-white hover:bg-blue-600">
			Add New Member
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
							Name
						</th>
						<th
							class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
						>
							Email
						</th>
						<th
							class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
						>
							Phone
						</th>
						<th
							class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500"
						>
							Actions
						</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-gray-200 bg-white">
					{#each members as member}
						<tr>
							<td class="whitespace-nowrap px-6 py-4"
								>{member.first_name + ' ' + member.last_name}</td
							>
							<td class="whitespace-nowrap px-6 py-4">{member.email}</td>
							<td class="whitespace-nowrap px-6 py-4">{member.phone || '-'}</td>
							<td class="whitespace-nowrap px-6 py-4">
								<a href="/members/{member.id}" class="text-blue-600 hover:text-blue-900">Edit</a>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
