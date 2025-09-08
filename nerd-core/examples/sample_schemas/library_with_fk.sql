-- Library Management System with Foreign Keys
CREATE TABLE authors (
    id INT PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    birth_date DATE,
    nationality VARCHAR(50)
);

CREATE TABLE publishers (
    id INT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    address TEXT,
    founded_year INT
);

CREATE TABLE categories (
    id INT PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    description TEXT
);

CREATE TABLE books (
    id INT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    isbn VARCHAR(20) UNIQUE,
    publication_date DATE,
    pages INT,
    author_id INT NOT NULL,
    publisher_id INT NOT NULL,
    category_id INT NOT NULL
);

CREATE TABLE members (
    id INT PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE,
    phone VARCHAR(20),
    membership_date DATE DEFAULT CURRENT_DATE
);

CREATE TABLE loans (
    id INT PRIMARY KEY,
    book_id INT NOT NULL,
    member_id INT NOT NULL,
    loan_date DATE DEFAULT CURRENT_DATE,
    due_date DATE NOT NULL,
    return_date DATE,
    status VARCHAR(20) DEFAULT 'active'
);

-- Note: The current parser focuses on CREATE TABLE statements
-- In a full implementation, these foreign key relationships would be detected:
-- books.author_id -> authors.id
-- books.publisher_id -> publishers.id  
-- books.category_id -> categories.id
-- loans.book_id -> books.id
-- loans.member_id -> members.id