-- Add migration script here
CREATE TABLE IF NOT EXISTS todo_category (
	todo_id BIGINT UNSIGNED NOT NULL,
	category_id BIGINT UNSIGNED NOT NULL,

	PRIMARY KEY (todo_id, category_id),
	FOREIGN KEY (todo_id) REFERENCES todo.`todo` (id),
	FOREIGN KEY (category_id) REFERENCES todo.category (id)
);
