# Codernic CI/CD Enterprise Showcase

Welcome to the **Codernic CI/CD Demo Repository**!

This repository serves as a live, multi-language sandbox proving the vast covering ability and advanced security enforcement of [Galileus CI](https://codernic.dev), our premier AI-powered DevSecOps agent.

Visit [Codernic.dev](https://codernic.dev) to learn how you can integrate Galileus CI into your enterprise workflows to permanently eradicate rubber-stamped reviews, leaked secrets, and architectural degradation.

## Repository Structure

This monorepo mimics a standard enterprise application, spanning across four distinct technology stacks to demonstrate Galileus CI's universal comprehension:

- `backend-rust/`: The core payment processing engine written in Rust.
- `frontend-react/`: The client application and UI middleware written in Node.js/TypeScript.
- `infrastructure/`: The GitOps state descriptors (Terraform).
- `data-scripts/`: Python pipelines for enterprise data processing.

## The "Crime Scenes" (How to Dogfood)

To see *exactly* how Galileus CI catches what traditional linters and overloaded senior developers miss, explore the Pull Requests in this repository. We have deliberately constructed four "Crime Scene" branches:

### 1. The "Rubber Stamp" Blindspot (`demo/scene-1-rubber-stamp`)
* **The Vulnerability**: A subtle asynchronous race condition was introduced in the frontend middleware.
* **Why it matters**: Senior devs often skim clean-looking code, missing logical flaws.
* **Galileus Action**: Blocked by the *SemanticIntentReview* node.

### 2. The GitOps Disaster (`demo/scene-2-gitops-disaster`)
* **The Vulnerability**: A destructive SQL/Terraform state deletion without snapshot verifications.
* **Why it matters**: CI pipelines blindly executing state-destructive commands can wipe production data.
* **Galileus Action**: Blocked by the *Sovereign R&D Governance* node.

### 3. The "Too Big To Review" Monolith (`demo/scene-3-monolithic-god-function`)
* **The Vulnerability**: A 500+ line monolithic "God Function" violating Clean Architecture.
* **Why it matters**: Overloaded reviewers approve massive PRs out of exhaustion, spiking technical debt.
* **Galileus Action**: Blocked by the *StaticAnalysis* node for SRP violations.

### 4. The Buried Secret (`demo/scene-4-leaked-secrets`)
* **The Vulnerability**: A hardcoded AWS Secret Access Key in a Python configuration payload.
* **Why it matters**: Supply chain vulnerabilities are the leading cause of enterprise breaches.
* **Galileus Action**: Quarantined and blocked immediately.

### 5. Enterprise Remediation (`demo/scene-5-enterprise-remediation`)
* **The Proof**: The perfectly refactored, SOLID-compliant code that successfully passes the Galileus pipeline with flying colors.

---

*Powered by [Codernic Enterprise](https://codernic.dev).*
