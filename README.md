# sqlBarser
With the rising need of monitoring data usage via SQL query and ddl. We (Bifroz Lab) want to build a SQL parser that is just not a parser. Usually, a parser only returns AST but we also want to return acessing table and columns from query.

More specifically, we want to build a parser with following capabilities:
- Multiple SQL dialects (Posgres, BigQuery, Trino, ...).
- Accessing tables and columns of each tables.
- Data linage of the query (e.g which columns derivied the final columns).
- Multiple language interface (Rust, Python, Java, ...)


