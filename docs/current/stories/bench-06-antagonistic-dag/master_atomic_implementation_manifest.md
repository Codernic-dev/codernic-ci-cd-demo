# Atomic Implementation Steps - bench-06-antagonistic-dag

| Step ID | Code Slot | Target Files | Dependencies | Description |
|---|---|---|---|---|
| `STEP-DAG-CORE` | `dag_core_nodes` | `backend-rust/src/dag_nodes.rs` | `None` | Structure de noeuds de dependances et stockage par identifiants |
| `STEP-DAG-KAHN` | `dag_kahn_solver` | `backend-rust/src/dag_solver.rs` | `STEP-DAG-CORE` | Algorithme de Kahn pur std pour resolution lineaire ordonnee |
| `STEP-DAG-CYCLE` | `dag_cycle_tracer` | `backend-rust/src/dag_tracer.rs` | `STEP-DAG-KAHN` | Algorithme de backtracking pour reconstruction de la boucle cyclique |
| `STEP-DAG-PTP` | `dag_antagonistic_ptp` | `backend-rust/tests/test_antagonistic_dag_ptp.rs` | `STEP-DAG-CYCLE` | Suite de tests physiques adverses sur graphes denses et boucles complexes |
