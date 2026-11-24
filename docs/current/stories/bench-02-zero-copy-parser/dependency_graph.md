# Dependency Graph (DAG) - bench-02-zero-copy-parser

```mermaid
graph TD
    ["STEP-ZC-ERRORS"]
    ["STEP-ZC-ERRORS"] --> ["STEP-ZC-HEADER"]
    ["STEP-ZC-HEADER"] --> ["STEP-ZC-PAYLOAD"]
```
