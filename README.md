# sqlformat-py
A Python wrapper around sqlformat-rs. (https://github.com/shssoichiro/sqlformat-rs)

```Python
>>> import sqlformat
>>> print(sqlformat.format("SELECT * FROM TABLE_NAME"))
SELECT
  *
FROM
  TABLE_NAME
```
