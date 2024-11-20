INSERT INTO BOOKS (title, author, category, status) VALUES
('The Great Gatsby', 'F. Scott Fitzgerald', 'Fiction', 'available'),
('To Kill a Mockingbird', 'Harper Lee', 'Fiction', 'borrowed'),
('Introduction to SQL', 'John Smith', 'Technology', 'available'),
('The Art of Programming', 'Jane Doe', 'Technology', 'available');

INSERT INTO MEMBERS (first_name, last_name, email, phone) VALUES
('John', 'Doe', 'john@email.com', '123-456-7890'),
('Jane', 'Smith', 'jane@email.com', '123-456-7891'),
('Bob', 'Wilson', 'bob@email.com', '123-456-7892');

INSERT INTO BORROWINGS (book_id, member_id, borrow_date, return_date) VALUES
(2, 1, '2023-01-15', NULL),
(1, 2, '2023-01-10', '2023-01-20'),
(3, 3, '2023-02-01', '2023-02-15');
