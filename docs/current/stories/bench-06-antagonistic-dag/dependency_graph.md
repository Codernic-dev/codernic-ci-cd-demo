# Dependency Graph (DAG) - bench-06-antagonistic-dag

```mermaid
graph TD
    ["STEP-DAG-CORE"]
    ["STEP-DAG-CORE"] --> ["STEP-DAG-KAHN"]
    ["STEP-DAG-KAHN"] --> ["STEP-DAG-CYCLE"]
    ["STEP-DAG-CYCLE"] --> ["STEP-DAG-PTP"]
```
