# Feature Specification Blueprint (FSB)
## Niveau 2 : Parseur Binaire Zero-Copy avec Durees de Vie (Lifetimes)

```yaml
story_slug: "bench-02-zero-copy-parser"
version: "1.0.0"
target_package: "backend-rust"
category: "BENCHMARK_LEVEL_2_ZERO_ALLOC"
complexity: "MEDIUM"
estimated_steps: 3
```

### 1. Contexte & Enjeux

Ce banc d'essai eprouve la capacite des modeles a respecter les contraintes de memoire et de duree de vie Rust :
- Interdiction absolue de l'allocation heap dans la boucle de decodage (`ZeroHeap` invariant : aucun `String`, `Vec` ou `Box` permis dans les structures de parsing).
- Utilisation stricte de tranches empruntees avec lifetime explicite (`&'a [u8]` et `&'a str`).
- Gestion exhaustive des erreurs par un enum sans panique ni `unwrap()`.

### Pieges Classiques pour les LLMs
1. **Hallucination de Crates** : Tendance a vouloir ajouter `serde`, `nom` ou `byteorder` alors que `Cargo.toml` n'autorise que la librairie standard.
2. **Fuite d'Allocation** : Tendance a appeler `.to_string()` ou `String::from_utf8_lossy()` pour contourner les erreurs de lifetime du borrow-checker.
3. **Borrow Checker Violations** : Invalidation de references lors de multiples emprunts mutables simultanes.

### 2. Table des Etapes Atomiques

| Step ID | Code Slot | Target Files | Dependencies | Description |
| :--- | :--- | :--- | :--- | :--- |
| STEP-ZC-ERRORS | zc_parse_errors | backend-rust/src/zero_copy_errors.rs | None | Enum d'erreurs de parsing exhaustif sans allocation |
| STEP-ZC-HEADER | zc_parse_header | backend-rust/src/zero_copy_header.rs | STEP-ZC-ERRORS | Parseur zero-copie d'en-tetes de paquets reseau |
| STEP-ZC-PAYLOAD | zc_parse_payload | backend-rust/src/zero_copy_payload.rs | STEP-ZC-HEADER | Validateur de trame complete avec slices immutables |

### 3. Criteres de Verification CompileGate

- Compilation stricte `cargo check` avec `#![deny(clippy::all)]`.
- Aucune dependance externe ajoutee au Cargo.toml.
- Tests physiques validant la non-allocation et le comportement sur trames corrompues ou tronquees.
