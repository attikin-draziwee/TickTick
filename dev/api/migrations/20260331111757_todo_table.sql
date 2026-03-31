-- Add migration script here
CREATE TABLE IF NOT EXISTS `todo` (
	id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
	user_id BIGINT UNSIGNED NOT NULL,
	title VARCHAR(350) NOT NULL,
	body TEXT,

	PRIMARY KEY (id),
	FOREIGN KEY (user_id) REFERENCES todo.`user` (id)
) DEFAULT CHARSET=utf8mb4 COLLATE utf8mb4_unicode_ci;
