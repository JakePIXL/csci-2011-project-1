export enum Order {
	ASC = 'asc',
	DESC = 'desc'
}

export enum Status {
	Available = 'available',
	Borrowed = 'borrowed',
	All = 'all'
}

export interface Book {
	id: number;
	title: string;
	author: string;
	category?: string;
	status: Status;
}

export interface Member {
	id: number;
	first_name: string;
	last_name: string;
	email: string;
	phone?: string;
}

export interface Borrowing {
	id: number;
	book_id?: number;
	member_id?: number;
	borrow_date: string;
	return_date?: string;
}

export interface BorrowedBook {
	id: number;
	title: string;
	author: string;
	borrower: string;
	borrower_id: number;
	borrow_date: string;
	return_date?: string;
	status: Status;
}
