CREATE TABLE `user` (
    `id` TEXT PRIMARY KEY NOT NULL,
    `username` TEXT UNIQUE NOT NULL,
    `password` BLOB NOT NULL
);

CREATE INDEX `user_i0` ON `user` (`username`);

CREATE TABLE `account` (
    `id` TEXT PRIMARY KEY NOT NULL,
    `name` TEXT NOT NULL,
    `alias` TEXT NOT NULL,
    `owner` TEXT NOT NULL REFERENCES `user` (`id`),
    `kind` TEXT NOT NULL
);

CREATE INDEX `account_i0` ON `account` (`owner`);

CREATE TABLE `transaction` (
    `id` TEXT PRIMARY KEY NOT NULL,
    `account` TEXT NOT NULL REFERENCES `account` (`id`),
    `date` DATE NOT NULL,
    `action` TEXT NOT NULL
);

CREATE INDEX `transaction_i0` ON `transaction` (`account`);

CREATE INDEX `transaction_i1` ON `transaction` (`date`);
