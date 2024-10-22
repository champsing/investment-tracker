INSERT INTO
    `Configs` (`key`, `value`)
VALUES
    (?1, ?2)
ON CONFLICT (`key`) DO UPDATE
SET
    `value` = ?2
WHERE
    `key` = ?1;