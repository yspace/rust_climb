How can i get the latest record in database based on datetime?

- https://www.sqlitetutorial.net/sqlite-date-functions/sqlite-datetime-function/

参考：https://kerkour.com/high-performance-rust-with-sqlite
有  created_at: chrono::DateTime<chrono::Utc>, 的使用

sqlite 中 好像没有日期类型 或许用int  unix timestamp类型比较好吧 
网上很多例子 都是用TEXT类型的  对于排序感觉不好吧！
~~~sql

CREATE TABLE "projects" (
	"id"	INTEGER NOT NULL,
	"title"	TEXT NOT NULL,
	"detail"	TEXT NOT NULL,
    "download_url" TEXT NOT NULL,
    "status"	INTEGER NOT NULL DEFAULT 0,


	"created_at"	TEXT NOT NULL DEFAULT (DATETIME(''now'', ''localtime'')),
	"updated_at"	TEXT NOT NULL DEFAULT (DATETIME(''now'', ''localtime'')),
	PRIMARY KEY("id")
);

CREATE TRIGGER trigger_projects_updated_at AFTER UPDATE ON projects
BEGIN
    UPDATE projects SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
END;
~~~

另一个例子

~~~sql

CREATE TABLE `Playlist` (
    `playlistId` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, 
    `name` TEXT, 
    `description` TEXT, 
    `category` TEXT DEFAULT 'normal', 
    `createdTime` TEXT DEFAULT CURRENT_TIMESTAMP, 
    `lastModifiedTime` TEXT DEFAULT CURRENT_TIMESTAMP
)
~~~~

https://github.com/diesel-rs/diesel/blob/2.0.x/examples/sqlite/all_about_inserts/migrations/2017-12-16-173626_create_users/up.sql

这里有例子 ：

~~~sql

CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  hair_color TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
~~~

类型多了？  下面这个
~~~sql

-- SCHEMA

-- table
CREATE TABLE somethings (
  id integer PRIMARY KEY AUTOINCREMENT  NOT NULL
  --
  ,name   text
  ,email  varchar(255)  NOT NULL
  --
  ,created_at datetime  NOT NULL  DEFAULT current_timestamp
  ,updated_at datetime  NOT NULL  DEFAULT current_timestamp
  -- soft delete
  ,deleted_at datetime
);

-- index (deleted_at)
CREATE INDEX idx_somethings_deleted_at
    ON somethings (deleted_at ASC);

-- index (email)
CREATE UNIQUE INDEX idx_somethings_email
  ON somethings (email ASC)
  WHERE deleted_at IS NULL;

-- trigger (updated_at)
CREATE TRIGGER tg_somethings_updated_at
AFTER UPDATE
ON somethings FOR EACH ROW
BEGIN
  UPDATE somethings SET updated_at = current_timestamp
    WHERE id = old.id;
END;

--

-- STUFF

--


-- SELECT (lookup)
--
SELECT * FROM somethings WHERE deleted_at IS NULL;

-- CREATE RECORDS
--
INSERT INTO somethings (name, email) VALUES
    ('jho', 'jho@example.com'),
    ('bob', 'bob@example.com'),
    ('alice', 'alice@example.com');

-- cehck
SELECT * FROM somethings WHERE deleted_at IS NULL;

-- EMAIL UNIQUINESS (SHOULD FAIL)
--
INSERT INTO somethings (name, email) VALUES
    ('jho', 'jho@example.com');

--
-- SOFT DELETE (jho for example)
UPDATE somethings SET deleted_at = current_timestamp
    WHERE email = 'jho@example.com'
    AND deleted_at IS NULL;

-- cehck
SELECT * FROM somethings WHERE deleted_at IS NULL;

-- INSERT jho AGAIN (SHOULD PASS)
--
INSERT INTO somethings (name, email) VALUES
    ('jho', 'jho@example.com');

-- cehck
SELECT * FROM somethings WHERE deleted_at IS NULL;

--
-- TOUCH (jho for example)
UPDATE somethings SET updated_at = current_timestamp
    WHERE email = 'jho@example.com'
    AND deleted_at IS NULL;

-- cehck
SELECT * FROM somethings WHERE deleted_at IS NULL;
~~~