# sqlformat-py

[![Build](https://github.com/brianjbuck/sqlformat-py/actions/workflows/build_wheels.yml/badge.svg?branch=master)](https://github.com/brianjbuck/sqlformat-py/actions/workflows/build_wheels.yml)

A Python wrapper around sqlformat-rs. (https://github.com/shssoichiro/sqlformat-rs)

Supported Python Versions: 3.7+

# Usage:

```Python
>>> import sqlformat
>>> print(sqlformat.format("SELECT * FROM TABLE_NAME"))
SELECT
  *
FROM
  TABLE_NAME
```
