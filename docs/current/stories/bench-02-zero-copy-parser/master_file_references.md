# Master File References - bench-02-zero-copy-parser

| Action | Relative Path | Primary Code Slot | Description |
|---|---|---|---|
| **CREATE** | `backend-rust/src/zero_copy_errors.rs` | `zc_parse_errors` | Enum d'erreurs de parsing exhaustif sans allocation |
| **CREATE** | `backend-rust/src/zero_copy_header.rs` | `zc_parse_header` | Parseur zero-copie d'en-tetes de paquets reseau |
| **CREATE** | `backend-rust/src/zero_copy_payload.rs` | `zc_parse_payload` | Validateur de trame complete avec slices immutables |
