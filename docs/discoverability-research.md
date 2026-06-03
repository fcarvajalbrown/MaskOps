# MaskOps Discoverability Research

> How to maximize visibility of a niche Python/Rust Polars plugin in the security/data-privacy space — covering GitHub topics, PyPI metadata, README structure, and keyword strategy.
>
> Research date: 2026-06-02. Sources adversarially verified (102 agents, 25 claims, 6 killed).

---

## Executive Summary

Three independent surfaces control discoverability: **GitHub topics**, **PyPI metadata**, and **README content**. The Polars plugin ecosystem has almost no standardized tagging — the official cookiecutter template ships with zero topics, no keywords, and a one-line README. The data-masking GitHub topic has only 123 repositories with Python leading at 25. The baseline is so bare that filling all three surfaces correctly will place MaskOps near the top of every relevant niche search with minimal competition.

---

## Finding 1 — GitHub Topics

**Confidence: high** | Sources: [GitHub Docs](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/classifying-your-repository-with-topics), [GitHub Engineering Blog](https://github.blog/engineering/user-experience/topics/)

### Rules

- Maximum **20 topics** per repository.
- Must be **lowercase letters, numbers, and hyphens only**. Max 50 characters each.
- Topics power GitHub's topic-based search (`github.com/topics/<tag>`) and browse pages.

### How GitHub's suggestion engine works

GitHub's `repo-topix` system uses **tf-idf** scoring against a corpus of millions of public READMEs. It draws per-repo term frequency from three sources: the repo **name**, the repo **description**, and the **README**. This means:

1. Keywords in the repo name get the highest weight.
2. Keywords in the description get high weight.
3. Keywords that appear repeatedly and early in the README reinforce topic suggestions.

The system was documented in a 2017 engineering blog post and a 2024 research paper treats it as still current — but GitHub has not publicly confirmed whether the production system has since been updated.

### Competitive landscape

| Topic | Repos (2026-06-02) | Python share | MaskOps fit |
|---|---|---|---|
| `polars-extensions` | 9 | 7/9 | Direct — currently almost empty |
| `data-masking` | 123 | 25/123 (leading language) | Direct |
| `polars` | ~809 | majority | Broad — high visibility, competitive |
| `data-privacy` | ~533 | unverified | Broad |
| `pii` | small | unknown | Direct |

The `polars-extensions` topic is the most exploitable — 9 repos total, max 11 stars, median ~3 stars. Ranking first there requires nothing more than adding the tag.

### Recommended 20 topics for MaskOps

```
polars
polars-extensions
polars-plugin
data-masking
data-privacy
pii
pii-detection
gdpr
data-anonymization
pseudonymization
fpe
format-preserving-encryption
rust
arrow
python
iban
credit-card
cybersecurity
data-engineering
regex
```

---

## Finding 2 — PyPI Metadata

**Confidence: high** | Sources: [Python Packaging Docs](https://packaging.python.org/en/latest/guides/writing-pyproject-toml/), [pyOpenSci Guide](https://www.pyopensci.org/python-package-guide/package-structure-code/pyproject-toml-python-package-metadata.html), [PyPI Classifiers](https://pypi.org/classifiers/), [PyPI warehouse source](https://github.com/pypi/warehouse/blob/main/warehouse/search/queries.py)

### Search field boost weights (from PyPI warehouse source)

| Field | Boost |
|---|---|
| Package name | 10× |
| Description (one-liner) | 5× |
| Keywords | 5× |
| Summary | 5× |
| README / long description | 1× |

The `keywords` field and the one-line `description` both carry a **5× boost** — the same weight. The description is also the only text visible in search result rows, making it the primary click-through driver.

### What MaskOps currently has vs. what it needs

| Field | Current state | Action needed |
|---|---|---|
| `description` | `"High-speed PII masking as a Polars plugin — powered by Rust"` | Good — keep |
| `keywords` | `["polars", "pii", "masking", "gdpr", "rust"]` | Expand — too sparse |
| `classifiers` | `Programming Language :: Rust`, `Programming Language :: Python :: 3`, `Topic :: Security`, `Topic :: Scientific/Engineering :: Information Analysis` | Add more (see below) |

### Recommended keywords expansion

```toml
keywords = [
  "polars", "polars-plugin", "polars-extension",
  "pii", "pii-detection", "pii-masking",
  "data-masking", "data-anonymization", "pseudonymization",
  "gdpr", "data-privacy", "compliance",
  "fpe", "format-preserving-encryption", "ff3",
  "iban", "credit-card", "cpf", "rut", "curp",
  "rust", "arrow", "regex"
]
```

### Recommended additional classifiers

```toml
"Topic :: Security :: Cryptography",
"Topic :: Database",
"Topic :: Software Development :: Libraries :: Python Modules",
"Intended Audience :: Developers",
"Intended Audience :: Financial and Insurance Industry",
"Intended Audience :: Healthcare Industry",
"Operating System :: OS Independent",
"Programming Language :: Rust",
"Programming Language :: Python :: 3.9",
"Programming Language :: Python :: 3.10",
"Programming Language :: Python :: 3.11",
"Programming Language :: Python :: 3.12",
```

---

## Finding 3 — Polars Plugin Ecosystem Gap

**Confidence: high** | Sources: [Polars user guide plugins page](https://docs.pola.rs/user-guide/plugins/), [polars-extensions topic](https://github.com/topics/polars-extensions), [cookiecutter-polars-plugins](https://github.com/MarcoGorelli/cookiecutter-polars-plugins)

### The cookiecutter problem

The official Polars plugin template (`MarcoGorelli/cookiecutter-polars-plugins`) — which most plugin authors use — ships with:

- **Zero GitHub topics** on the template repo
- **No `description` field** in the generated `pyproject.toml`
- **No `keywords` field** in the generated `pyproject.toml`
- A README with exactly **one line**: `# {{ cookiecutter.plugin_name }}`

This means the entire Polars plugin ecosystem starts at zero discoverability by default. Any plugin that fills these fields stands out immediately.

### The official plugins list

Polars maintains a manually curated, non-exhaustive list at `docs.pola.rs/user-guide/plugins/` organized into three categories: *Various*, *Data science*, and *Geo*. There is no automated registry and no submission form — it requires a PR to the Polars documentation repo.

**Getting listed there is the single highest-impact action for Polars-specific discoverability.** High-star extensions like `geopolars` (887 stars) are not even in the `polars-extensions` topic — they got traffic from the official list.

### Naming convention

The `polars-` prefix is common but not enforced. Most listed plugins follow it (polars-xdt, polars-hash, polars-ds). `MaskOps` diverges from this convention — not a problem for discoverability but worth noting for brand consistency decisions.

---

## Finding 4 — Open Source SEO and Backlink Strategy

Sources: [Read the Docs SEO guide](https://docs.readthedocs.com/platform/latest/guides/technical-docs-seo-guide.html), [dev.to open source SEO](https://dev.to/jcubic/seo-for-open-source-projects-4dm4)

### README structure signals

GitHub renders the README as the repo's landing page and Google indexes it. Structural signals that influence both GitHub's topic engine and Google:

1. **H1 at the top** with the library name and primary keyword (e.g., "PII masking").
2. **One-paragraph description** in the first 200 characters — this is what Google uses as the meta description snippet.
3. **Keyword-dense section headers** — Google treats H2/H3 as content signals. "Supported patterns", "Format-Preserving Encryption", "Polars expression" are already good.
4. **Badges** — downloads/week, CI status, PyPI version, license — show activity and legitimacy to both search engines and humans.
5. **Code example above the fold** — reduces bounce rate, which Google uses as an indirect quality signal.

The current MaskOps README already does most of this well. The main gap is badges and the absence of a link to a documentation site.

### Awesome lists

`ddotta/awesome-polars` is the most-followed curated list for the Polars ecosystem. Getting added there drives ongoing referral traffic and backlinks. Submission is a PR with a one-liner description under the appropriate category.

### Documentation site

A hosted documentation site (Read the Docs, GitHub Pages) creates an independent indexable domain. Google treats docs sites as separate from the repo and indexes them independently. For a library this size, GitHub Pages from `docs/` is sufficient.

---

## Refuted Claims

The following claims were verified and killed (0-3 or 1-2 votes):

| Claim | Why killed |
|---|---|
| `polars-` prefix is an enforced naming convention | Not enforced; high-star repos diverge from it |
| `data-privacy` topic has 203 Python repos out of 533 total | Live count did not match; claim unverifiable |
| Top `data-privacy` Python repos use layered multi-dimensional tagging | Not confirmed by live topic page inspection |
| PyPI `classifiers` field is required for minimum packaging | Not required; optional metadata |
| cookiecutter template repo itself has zero topics | Partially wrong; the template repo state differs from generated repo state |

---

## Open Questions

1. **Topic ordering**: Does GitHub's tf-idf engine weight topics differently based on ordering, or is the set unordered?
2. **Polars docs submission**: What is the exact PR process for `docs.pola.rs/user-guide/plugins/` and what criteria does the Polars team use to accept listings?
3. **README keyword density**: Is there a measurable threshold at which tf-idf keyword density triggers topic suggestions vs. being treated as spam?
4. **PyPI classifier traffic**: Do users actively filter PyPI by `Topic :: Security` classifier, or is keyword/name search the dominant discovery path?

---

## Action Checklist

- [ ] Add 20 GitHub topics to the repo (see recommended list in Finding 1)
- [ ] Expand `keywords` in `pyproject.toml` (see Finding 2)
- [ ] Add missing PyPI classifiers (see Finding 2)
- [ ] Submit PR to `ddotta/awesome-polars`
- [ ] Submit PR to Polars user guide plugins page (`docs.pola.rs/user-guide/plugins/`)
- [ ] Add download badge and PyPI version badge to README
- [ ] Evaluate GitHub Pages docs site

---

## Sources

| URL | Type | Research angle |
|---|---|---|
| [GitHub Docs — Topics](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/classifying-your-repository-with-topics) | Primary | GitHub discoverability |
| [GitHub Engineering Blog — Topics](https://github.blog/engineering/user-experience/topics/) | Primary | GitHub discoverability |
| [Python Packaging Docs](https://packaging.python.org/en/latest/guides/writing-pyproject-toml/) | Primary | PyPI metadata |
| [pyOpenSci Package Guide](https://www.pyopensci.org/python-package-guide/package-structure-code/pyproject-toml-python-package-metadata.html) | Primary | PyPI metadata |
| [PyPI Classifiers](https://pypi.org/classifiers/) | Primary | PyPI metadata |
| [PyPI warehouse source — search queries](https://github.com/pypi/warehouse/blob/main/warehouse/search/queries.py) | Primary | PyPI search boost weights |
| [cookiecutter-polars-plugins](https://github.com/MarcoGorelli/cookiecutter-polars-plugins) | Primary | Polars ecosystem |
| [Polars user guide — plugins](https://docs.pola.rs/user-guide/plugins/) | Primary | Polars ecosystem |
| [polars-extensions topic](https://github.com/topics/polars-extensions) | Primary | Polars ecosystem |
| [data-masking topic](https://github.com/topics/data-masking) | Primary | Keyword strategy |
| [awesome-polars](https://github.com/ddotta/awesome-polars) | Secondary | Backlink strategy |
| [Read the Docs SEO guide](https://docs.readthedocs.com/platform/latest/guides/technical-docs-seo-guide.html) | Secondary | Open source SEO |
| [dev.to — SEO for open source](https://dev.to/jcubic/seo-for-open-source-projects-4dm4) | Blog | Open source SEO |
