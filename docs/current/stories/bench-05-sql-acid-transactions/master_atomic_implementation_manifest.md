# Atomic Implementation Steps - bench-05-sql-acid-transactions

| Step ID | Code Slot | Target Files | Dependencies | Description |
|---|---|---|---|---|
| `STEP-SQL-DDL` | `sql_schema_ddl` | `data-scripts/schema_ledger.sql` | `None` | Schema DDL 100% idempotent avec cles etrangeres et triggers d'immuabilite WORM |
| `STEP-SQL-ENGINE` | `sql_ledger_engine` | `data-scripts/ledger_engine.py` | `STEP-SQL-DDL` | Moteur d'insertion transactionnelle ACID avec requetes preparees et rollbacks |
| `STEP-SQL-TESTS` | `sql_ledger_tests` | `data-scripts/test_ledger_acid.py` | `STEP-SQL-ENGINE` | Suite de tests physiques verifiant le rejet d'UPDATE, le rollback et la non-injection |
