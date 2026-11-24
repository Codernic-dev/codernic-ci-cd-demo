# Dependency Graph (DAG) - bench-05-sql-acid-transactions

```mermaid
graph TD
    ["STEP-SQL-DDL"]
    ["STEP-SQL-DDL"] --> ["STEP-SQL-ENGINE"]
    ["STEP-SQL-ENGINE"] --> ["STEP-SQL-TESTS"]
```
