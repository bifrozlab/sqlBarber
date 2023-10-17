With the rising need of monitoring data usage via SQL query and ddl. We (Bifroz Lab) want to build a SQL parser that is just not a parser.
Usually, a parser only returns AST but we also want to return acessing table and columns from query.

More specifically, we want to build a _parser_ with following capabilities:

 - Supports multiple SQL dialects (Posgres, BigQuery, Trino, Spark).
 - Can return accessing tables and columns of each tables.
 - Can return Data linage of the query (e.g which columns derivied the final columns). 
 - Supports multiple language interface (at least Python and Java) so that we can plug it in many different usecases.

For more details of the current landscape of SQL Parser and the problem we are trying to tackle, you should take a view at these blogs.
 - https://hackernoon.com/14-open-source-sql-parsers
 - https://github.com/DTStack/dt-sql-parser
 - https://sonra.io/sql-parser/sql-parser-use-cases/
