# Dependency Graph (DAG) - bench-03-state-machine

```mermaid
graph TD
    ["STEP-SM-STATES"]
    ["STEP-SM-STATES"] --> ["STEP-SM-TRANSITIONS"]
    ["STEP-SM-TRANSITIONS"] --> ["STEP-SM-REGISTRY"]
    ["STEP-SM-REGISTRY"] --> ["STEP-SM-PTP"]
```
