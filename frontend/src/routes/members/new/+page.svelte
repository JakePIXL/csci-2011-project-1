<script lang="ts">
	import { goto } from '$app/navigation';

	let formData = {
		first_name: '',
		last_name: '',
		email: '',
		phone: ''
	};
	let error: string | null = null;
	let loading = false;

	async function handleSubmit(event: Event) {
		event.preventDefault();
		loading = true;
		error = null;

		try {
			const response = await fetch('/api/members/', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(formData)
			});

			if (response.ok) {
				goto('/members');
			} else {
				error = 'Failed to create member';
			}
		} catch (e) {
			error = 'Failed to create member';
			console.error(e);
		} finally {
			loading = false;
		}
	}
</script>

<div class="container mx-auto px-4 py-8">
	<div class="mb-6 flex items-center gap-4">
		<button
			onclick={() => window.history.back()}
			class="rounded-md bg-slate-200 px-2 py-1 text-xs hover:bg-slate-400"
		>
			back
		</button>
		<h1 class="text-2xl font-bold">New Member</h1>
	</div>

	<div class="rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
		<form onsubmit={handleSubmit} class="space-y-4">
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
				<div>
					<label for="first_name" class="block text-sm font-medium text-gray-700">
						First Name
					</label>
					<input
						type="text"
						id="first_name"
						bind:value={formData.first_name}
						required
						class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
					/>
				</div>

				<div>
					<label for="last_name" class="block text-sm font-medium text-gray-700"> Last Name </label>
					<input
						type="text"
						id="last_name"
						bind:value={formData.last_name}
						required
						class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
					/>
				</div>
			</div>

			<div>
				<label for="email" class="block text-sm font-medium text-gray-700"> Email </label>
				<input
					type="email"
					id="email"
					bind:value={formData.email}
					required
					class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
				/>
			</div>

			<div>
				<label for="phone" class="block text-sm font-medium text-gray-700"> Phone Number </label>
				<input
					type="tel"
					id="phone"
					bind:value={formData.phone}
					class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
				/>
			</div>

			{#if error}
				<div class="rounded-md bg-red-50 p-4">
					<div class="flex">
						<div class="flex-shrink-0">
							<svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
								<path
									fill-rule="evenodd"
									d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
									clip-rule="evenodd"
								/>
							</svg>
						</div>
						<div class="ml-3">
							<h3 class="text-sm font-medium text-red-800">{error}</h3>
						</div>
					</div>
				</div>
			{/if}

			<div class="flex justify-end">
				<button
					type="submit"
					disabled={loading}
					class="inline-flex items-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
				>
					{#if loading}
						<svg
							class="mr-2 h-4 w-4 animate-spin"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
						>
							<circle
								class="opacity-25"
								cx="12"
								cy="12"
								r="10"
								stroke="currentColor"
								stroke-width="4"
							/>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							/>
						</svg>
						Creating...
					{:else}
						Create Member
					{/if}
				</button>
			</div>
		</form>
	</div>
</div>
