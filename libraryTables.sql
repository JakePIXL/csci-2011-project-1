CREATE TABLE BOOKS (
    id INT PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(100) NOT NULL,
    author VARCHAR(100) NOT NULL,
    category VARCHAR(50),
    status VARCHAR(9) DEFAULT 'available'
);

CREATE TABLE MEMBERS (
    id INT PRIMARY KEY AUTO_INCREMENT,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    phone VARCHAR(20)
);

CREATE TABLE BORROWINGS (
    id INT PRIMARY KEY AUTO_INCREMENT,
    book_id INT,
    member_id INT,
    borrow_date DATE NOT NULL,
    return_date DATE,
    FOREIGN KEY (book_id) REFERENCES BOOKS(id) ON DELETE CASCADE,
    FOREIGN KEY (member_id) REFERENCES MEMBERS(id) ON DELETE CASCADE
);

CREATE OR REPLACE VIEW BORROWED_BOOKS AS
SELECT
    b.id,
    b.title,
    b.author,
    CONCAT(m.first_name, " ", m.last_name) AS borrower,
    m.id AS borrower_id,
    br.borrow_date,
    br.return_date,
    -- This section was hard to figure out, but thanks to stack overflow
    -- I was able to find a solution https://dub.sh/mix-collations
    CASE
        WHEN br.return_date IS NULL THEN 'borrowed' COLLATE utf8mb4_unicode_ci
        ELSE 'returned' COLLATE utf8mb4_unicode_ci
    END AS status
FROM BOOKS b
JOIN BORROWINGS br ON b.id = br.book_id
JOIN MEMBERS m ON m.id = br.member_id;
