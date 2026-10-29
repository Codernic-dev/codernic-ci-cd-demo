# Suite de Benchmarks et de Qualification Souveraine COE (CCM v2.1.0)

Ce dossier rassemble l'ensemble des **Feature Specification Blueprints (FSB)** concus pour eprouver, valider et stresser l'architecture de l'orchestrateur Codernic (COE), ses middlewares (Rust CompileGate, Frontend CFEM, SQL ACID), ainsi que son protocole de resilience et de supervision contradictoire face aux hallucinations et pieges cognitifs des LLMs.

---

## 1. Matrice de Qualification

| Niveau | Fichier Blueprint | Story Slug | Package Cible | Middleware & Composants Eprouves | Pieges & Echecs LLM Cibles |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Niveau 1 (Simple)** | `FSB_BENCH_01_CORE_PROBE.md` | `bench-01-core-probe` | `backend-rust` | Pipeline nominal, AST Scaffolder, Strike 0 | Derive verbeuse, dependances superflues |
| **Niveau 2 (Intermediaire)** | `FSB_BENCH_02_ZERO_COPY_PARSER.md` | `bench-02-zero-copy-parser` | `backend-rust` | CompileGate Clippy, Borrow Checker, Zero-Alloc | Hallucination de crates (`nom`, `serde`), fuites `.to_string()` |
| **Niveau 3 (Complexe)** | `FSB_BENCH_03_CONCURRENT_STATE_MACHINE.md` | `bench-03-state-machine` | `backend-rust` | Lock-Free Concurrency, Destubbing, Oracles Pirsig | Hallucination async (`tokio`), deadlocks, race conditions |
| **Niveau 4 (Frontend)** | `FSB_BENCH_04_FRONTEND_REACT_GOVERNANCE.md` | `bench-04-frontend-governance` | `frontend-react` | CompileGate Frontend (CFEM), TypeScript strict | Styles inline `style={{...}}`, types `any`, hooks cycliques |
| **Niveau 5 (SQL / Data)** | `FSB_BENCH_05_SQL_ACID_TRANSACTIONS.md` | `bench-05-sql-acid-transactions` | `data-scripts` | CompileGate SQL, DDL Idempotent, WORM Triggers | Injections SQL par f-strings, oubli de `PRAGMA foreign_keys`, oubli de rollback |
| **Niveau 6 (Extreme)** | `FSB_BENCH_06_ANTAGONISTIC_CIRCULAR_DAG.md` | `bench-06-antagonistic-dag` | `backend-rust` | Kahn DAG, Superviseur Contradictoire (Strikes 2 & 3) | Cycles infinis `Rc<RefCell>`, hallucination `petgraph`, echec sur cycle minimal |
| **Multi-Complexite** | `FSB_BENCH_PAYMENT_VALIDATOR.md` | `bench-payment-validator` | `backend-rust` | Pipeline complet end-to-end, Tests physiques PTP | Regles de velocite antifraude, idempotence cryptographique |

---

## 2. Guide d'Execution

Chaque blueprint peut etre execute soit de maniere interactive dans le cockpit TUI 3-colonnes, soit en ligne de commande headless.

### Mode Interactif (Cockpit TUI 3-Colonnes)
Lancez le cockpit en ciblant le repertoire de demonstration :
```bash
coe tui --root /home/tadeop/dev/Codernic-Workspace/codernic-ci-cd-demo
```
- Utilisez les fleches `[Haut/Bas]` pour selectionner le benchmark souhaite.
- Appuyez sur `[Enter]` ou `[L]` pour lancer l'execution.
- Observez le deroulement en direct :
  - **Colonne 1** : Etapes atomiques, avancement et audit WAL.
  - **Colonne 2** : Chaine de pensee (CoT `💭`) du modele Flash primaire.
  - **Colonne 3** : Sous-agents et superviseur contradictoire (`🤖`) en cas de difficulte (Strikes 2 et 3).
  - **Panneau Inferieur** : Metriques de tokens, economie de cache (%), latence et statut des frappes.

### Mode Headless (Execution Ligne de Commande)
Pour executer un banc d'essai directement :
```bash
# Exemple : Niveau 1 (Sonde nominale)
coe run --story bench-01-core-probe --bsf docs/FSB_BENCH_01_CORE_PROBE.md --root /home/tadeop/dev/Codernic-Workspace/codernic-ci-cd-demo

# Exemple : Niveau 6 (Stress DAG antagoniste)
coe run --story bench-06-antagonistic-dag --bsf docs/FSB_BENCH_06_ANTAGONISTIC_CIRCULAR_DAG.md --root /home/tadeop/dev/Codernic-Workspace/codernic-ci-cd-demo
```
