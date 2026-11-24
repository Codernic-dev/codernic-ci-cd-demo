# Feature Specification Blueprint (FSB)
## Niveau 4 : Middleware Frontend — Gouvernance Stricte React & TypeScript (CFEM)

```yaml
story_slug: "bench-04-frontend-governance"
version: "1.0.0"
target_package: "frontend-react"
category: "BENCHMARK_LEVEL_4_FRONTEND_CFEM"
complexity: "HIGH"
estimated_steps: 3
```

### 1. Contexte & Enjeux

Ce banc d'essai eprouve specifiquement le middleware Frontend (CFEM) et la gouvernance de code React / TypeScript :
- Creation d'un composant de surveillance de transactions en temps reel.
- Respect absolu de la charte de gouvernance Codernic Frontend :
  - **Zero Type `any`** : typage strict et exhaustif des etats, des proprietes et des evenements de saisie.
  - **Zero Style Inline** : interdiction formelle d'attributs `style={{...}}`. L'habillage doit reposer sur des classes CSS semantiques.
  - **Separation des Responsabilites** : isolation stricte de la logique metier dans un hook personnalise dedie (`useTransactionFilter`), dissociation des sous-composants atomiques (`TransactionRow`).
  - **Conformite Hooks React** : dependances exhaustives declarees dans `useMemo` et `useCallback` pour eviter les boucles de rendu.

### 2. Pieges & Modes d'Echec LLM Cibles

1. **Tentations des Styles Inline** : Les LLMs inserent spontanement des styles inline (`style={{ display: 'flex' }}`) pour positionner les elements, ce qui declenche un veto immediat du middleware CFEM.
2. **Echappement de Typage par `any`** : Les LLMs utilisent frequemment `(e: any)` ou `Record<string, any>` sur les callbacks de tri ou de filtrage.
3. **Destubbing Incomplet** : Abandon de commentaires `// TODO: integrate filtering` dans le hook metier.

### 3. Table des Etapes Atomiques

| Step ID | Code Slot | Target Files | Dependencies | Description |
| :--- | :--- | :--- | :--- | :--- |
| STEP-FE-TYPES | fe_audit_types | frontend-react/src/types_audit.ts | None | Interfaces TypeScript strictes sans aucun recours a `any` |
| STEP-FE-HOOK | fe_filter_hook | frontend-react/src/useTransactionFilter.ts | STEP-FE-TYPES | Hook metier encapsulant la logique de recherche et pagination |
| STEP-FE-WIDGET | fe_audit_widget | frontend-react/src/TransactionAuditWidget.tsx | STEP-FE-HOOK | Composant React pur sans style inline avec gestion d'etats |

### 4. Criteres de Verification CompileGate Frontend

- Validation TypeScript stricte : `npx tsc --noEmit` avec zero erreur.
- Audit CFEM : zero occurrence de `any` et zero occurrence de l'attribut `style=` dans les fichiers `.ts` et `.tsx`.
- Verification du destubbing effectif de toutes les fonctions de filtrage.
