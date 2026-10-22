# Master File References - bench-06-antagonistic-dag

| Action | Relative Path | Primary Code Slot | Description |
|---|---|---|---|
| **CREATE** | `backend-rust/src/dag_nodes.rs` | `dag_core_nodes` | Structure de noeuds de dependances et stockage par identifiants |
| **CREATE** | `backend-rust/src/dag_solver.rs` | `dag_kahn_solver` | Algorithme de Kahn pur std pour resolution lineaire ordonnee |
| **CREATE** | `backend-rust/src/dag_tracer.rs` | `dag_cycle_tracer` | Algorithme de backtracking pour reconstruction de la boucle cyclique |
| **CREATE** | `backend-rust/tests/test_antagonistic_dag_ptp.rs` | `dag_antagonistic_ptp` | Suite de tests physiques adverses sur graphes denses et boucles complexes |
