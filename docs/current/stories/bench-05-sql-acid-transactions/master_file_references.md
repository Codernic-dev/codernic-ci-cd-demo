# Master File References - bench-05-sql-acid-transactions

| Action | Relative Path | Primary Code Slot | Description |
|---|---|---|---|
| **CREATE** | `data-scripts/schema_ledger.sql` | `sql_schema_ddl` | Schema DDL 100% idempotent avec cles etrangeres et triggers d'immuabilite WORM |
| **CREATE** | `data-scripts/ledger_engine.py` | `sql_ledger_engine` | Moteur d'insertion transactionnelle ACID avec requetes preparees et rollbacks |
| **CREATE** | `data-scripts/test_ledger_acid.py` | `sql_ledger_tests` | Suite de tests physiques verifiant le rejet d'UPDATE, le rollback et la non-injection |
