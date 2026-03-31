-- Add migration script here
CREATE TABLE IF NOT EXISTS category (
	id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
	user_id BIGINT UNSIGNED NOT NULL,
	name VARCHAR(100) NOT NULL,

	PRIMARY KEY (id),
	FOREIGN KEY (user_id) REFERENCES todo.`user` (id),

	UNIQUE KEY (user_id, name)
) DEFAULT CHARSET=utf8mb4 COLLATE utf8mb4_unicode_ci;
