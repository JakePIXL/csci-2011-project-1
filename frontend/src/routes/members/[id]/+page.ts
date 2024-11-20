import type { BorrowedBook, Member } from '$lib/views';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
	const [memberRes, borrowingsRes] = await Promise.all([
		fetch(`/api/members/${params.id}`),
		fetch(`/api/borrows/${params.id}`)
	]);

	const member = (await memberRes.json()) as Member;
	const borrowings = (await borrowingsRes.json()) as BorrowedBook[];

	return {
		member,
		borrowings
	};
};
