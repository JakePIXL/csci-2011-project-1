import type { Book, Member } from '$lib/views';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
	const [memberRes, booksRes] = await Promise.all([
		fetch(`/api/members/${params.id}`),
		fetch(`/api/books/?status=available`)
	]);

	const member = (await memberRes.json()) as Member;
	const books = (await booksRes.json()) as Book[];

	return {
		member,
		books
	};
};
