# CSCI211 Project 1 - Library Management System

A simple CRUD (Create, Read, Update, Delete) application built with Rust, Actix-web, and SQLx for managing a basic library system.

## Overview

This project implements a basic library management system that allows librarians to manage books, members, and borrowing records. The application is built using Rust's Actix-web framework for the web server and SQLx for database operations.

## Features

- Manage Books:
  - Add new books
  - Update book information
  - Delete books
  - View book details
  - Track book status (available/borrowed)

- Manage Members:
  - Register new members
  - Update member information
  - Remove members
  - View member details

- Handle Borrowings:
  - Record book borrowings
  - Process book returns
  - Track borrowing history
  - View current borrowings

## Database Schema

The application uses a MariaDB database with the following structure:

### Books Table
- `id` (Primary Key)
- `title`
- `author`
- `category`
- `status`

### Members Table
- `id` (Primary Key)
- `name`
- `email`
- `phone`

### Borrowings Table
- `id` (Primary Key)
- `book_id` (Foreign Key)
- `member_id` (Foreign Key)
- `borrow_date`
- `return_date`

### Borrowings View
- `id`
- `book_id`
- `book_title`
- `borrower`
- `borrower_id`
- `borrow_date`
- `return_date`
- `status

## Technologies Used

- Rust
- Actix-web (Web Framework)
- SQLx (Database Library)
- MariaDB (Database)
- Serde (Serialization)
- Svelte/Kit (Frontend)

## API Endpoints

### Books
- `GET /api/books` - List all books with filtering options
  - Query parameters: `title`, `author`, `category`, `status`, `order_by`, `order`, `limit`, `page`
- `GET /api/books/{id}` - Get book details by ID
- `POST /api/books` - Add a new book
- `PUT /api/books/{id}` - Update book information
- `DELETE /api/books/{id}` - Delete a book

### Members
- `GET /api/members` - List all members with filtering options
  - Query parameters: `name`, `email`, `phone`, `order_by`, `order`, `limit`, `page`
- `GET /api/members/{id}` - Get member details by ID
- `POST /api/members` - Register a new member
- `PUT /api/members/{id}` - Update member information
- `DELETE /api/members/{id}` - Remove a member

### Borrowings
- `GET /api/borrows` - List all borrowings with filtering options
  - Query parameters: `status`, `order`
- `GET /api/borrows/{id}` - Get borrowing details by member ID
  - Query parameters: `status`, `order`
- `POST /api/borrows/{id}` - Create a new borrowing for a member
- `POST /api/borrows/return/` - Return a borrowed book
- `DELETE /api/borrows/{id}` - Delete a borrowing record (By Borrowing ID)

## ER Diagram

```mermaid
erDiagram
    BOOKS ||--o{ BORROWINGS : has
    MEMBERS ||--o{ BORROWINGS : makes

    BOOKS {
        int id PK
        string title
        string author
        string category
        string status
    }

    MEMBERS {
        int id PK
        string name
        string email
        string phone
    }

    BORROWINGS {
        int id PK
        int book_id FK
        int member_id FK
        date borrow_date
        date return_date
    }
```

## Usage

To run the program simply run the binary that was generated:

```bash
./projectone
```

You may need to change the connection string in the .env file to match your database configuration.

```text
DATABASE_URL=mariadb://username:password@localhost/library
```
