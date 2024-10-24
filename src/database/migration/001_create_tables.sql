CREATE TABLE `accounts` (
    `id` TEXT PRIMARY KEY NOT NULL,
    `acct` TEXT NOT NULL,
    `name` TEXT NOT NULL,
    `type` TEXT NOT NULL
);

CREATE TABLE `transactions` (
    `id` TEXT PRIMARY KEY NOT NULL,
    `account` TEXT NOT NULL REFERENCES `accounts` (id),
    `date` DATE NOT NULL,
    `action` TEXT NOT NULL
);

CREATE INDEX `transactions_i0` ON `transactions` (`date`);
