
[inserting One Billion Rows in SQLite Under A Minute](https://www.reddit.com/r/rust/comments/omsar3/inserting_one_billion_rows_in_sqlite_under_a/)

https://github.com/avinassh/fast-sqlite3-inserts


https://avi.im/blag/2021/fast-sqlite-inserts/
~~~sql

PRAGMA journal_mode = OFF;
PRAGMA synchronous = 0;
PRAGMA cache_size = 1000000;
PRAGMA locking_mode = EXCLUSIVE;
PRAGMA temp_store = MEMORY;
~~~
>> What do these do?

        Turning off journal_mode will result in no rollback journal, thus we cannot go back if any of the transactions fail. This disables the atomic commit and rollback capabilities of SQLite. Do not use this in production.
        By turning off synchronous, SQLite does not care about writing to disk reliably and hands off that responsibility to the OS. A write to SQLite, may not mean it is flushed to the disk. Do not use this in production.
        The cache_size specifies how many memory pages SQLite is allowed to hold in the memory. Do not set this to a high value in production.
        In EXCLUSIVE locking mode, the lock held by the SQLite connection is never released.
        Setting temp_store to MEMORY will make it behave like an in-memory database.


[参数意义见文档](https://www.sqlite.org/pragma.html)

