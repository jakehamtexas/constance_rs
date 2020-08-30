CREATE DATABASE test

CREATE TABLE simple_enum
(
	id INT IDENTITY(1,1) PRIMARY KEY,
	name VARCHAR(255)
)

INSERT INTO simple_enum
	(name)
VALUES
	('test1'),
	('test2')

SELECT *
FROM simple_enum