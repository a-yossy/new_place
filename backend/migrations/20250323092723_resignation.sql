CREATE TABLE
  resignation (
    id INT PRIMARY KEY AUTO_INCREMENT,
    retirement_date DATE NOT NULL,
    remaining_paid_leave_days INT UNSIGNED NOT NULL,
    created_at DATETIME NOT NULL
  );
