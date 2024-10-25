CREATE TABLE `users` (
    `id` TEXT PRIMARY KEY NOT NULL,
    `username` TEXT UNIQUE NOT NULL,
    `password` BLOB NOT NULL
);

CREATE INDEX `users_i0` ON `users` (`username`);

CREATE TABLE `accounts` (
    `id` TEXT PRIMARY KEY NOT NULL,
    `name` TEXT NOT NULL,
    `alias` TEXT NOT NULL,
    `owner` TEXT NOT NULL REFERENCES `users` (`id`),
    `kind` TEXT NOT NULL
);

CREATE INDEX `accounts_i0` ON `accounts` (`owner`);

CREATE TABLE `transactions` (
    `id` TEXT PRIMARY KEY NOT NULL,
    `account` TEXT NOT NULL REFERENCES `accounts` (`id`),
    `date` DATE NOT NULL,
    `action` TEXT NOT NULL
);

CREATE INDEX `transactions_i0` ON `transactions` (`account`);

CREATE INDEX `transactions_i1` ON `transactions` (`date`);
