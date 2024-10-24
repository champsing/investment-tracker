CREATE TABLE `Accounts` (
    `id` TEXT PRIMARY KEY NOT NULL,
    `acct` TEXT NOT NULL,
    `name` TEXT NOT NULL,
    `type` TEXT NOT NULL
);

CREATE TABLE `Transactions` (
    `id` TEXT PRIMARY KEY NOT NULL,
    `account` TEXT NOT NULL REFERENCES `Accounts` (id),
    `date` DATE NOT NULL,
    `action` TEXT NOT NULL
);

CREATE INDEX `Transactions_0` ON `Transactions` (`date`);
