Project 2 submission — htmd fuzzer
===================================

Specification: `### Fuzzer` in [`translation_approach.md`](translation_approach.md).
Reviews: [`llm-review.md`](llm-review.md). Pull request:
[bjones1/htmd#7](https://github.com/bjones1/htmd/pull/7).

### Contributions

* **Robert:** First draft, Jones oracle revision, LLM rounds 8–9 (Codex), team coordination.
* **Yerassyl:** LLM rounds 2, 5, 6, 10; `submission.md`; spec follow-ups after Jones review.
* **Alinur:** Complete fuzzer spec, LLM rounds 3 and 7.
* **Kurmanbek:** LLM rounds 1 and 4.

### Compiler test

Prompt: *Verify that you can compile and run the project in the current git repo.*

#### Robert

**Codex**, Rust **1.93.1**: `cargo build`, `cargo clippy -- --deny warnings`, and
`cargo test` (279 passed, 1 ignored) succeeded; examples `simple` and `table`
ran successfully.

#### Yerassyl

**Gemini 2.5 Flash**, Rust **1.99.1**: `cargo build` and `cargo test` (279 passed,
1 ignored) succeeded on **Generadium/htmd**.

#### Alinur

**Claude Sonnet 4.6**, Rust **1.93.1**: `cargo build` and `cargo test` succeeded.

#### Kurmanbek

**Grok 4.6**, Rust **1.93.1**: `cargo build` and `cargo test` succeeded.

### Spec review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 8 | Faithful default API; corpus seeds; oracle priorities 1–4 groundwork; generator table |
| 2 | 4 | Text escaping; fragment `<div>` context; empty/whitespace rows |
| 3 | 4 | String mutations; `#[ignore]` findings; fragment context |
| 4 | 4 | Verbatim payloads; tag-delimited mutations; Pure scope |
| 5 | 4 | Stream class at check time; priority 3 scope; DOM equivalence |
| 6 | 2 | Void tags in mutations; CI wall-clock cap |
| 7 | 0 | Final draft pass (no new spec edits) |
| 8 | 4 | Jones follow-up: child process; inline-root round-trip rules; whitespace; oracle table simplification (Robert / Codex) |
| 9 | 4 | UTF-8-safe deletes; pulldown options; whitespace detail (Robert / Codex) |
| 10 | 3 | Priority 3 cross-reference; generator table rows; whitespace collapse (Yerassyl / GPT-5.5) |

### Implementation review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| — | — | Fuzzer `cargo test` target not landed yet; spec and `submission.md` updated for instructor re-review. Task-list implementation remains in the same PR from earlier project work. |

### Last LLM spec review

Model: **GPT-5.5**. Prompt: `Review the section named "Fuzzer" in
translation_approach.md`.

*(No divergence findings after Round 10; see **Final review (post–Round 10)** in
`llm-review.md`.)*

### Last LLM implementation review

Not applicable until the fuzzer test target exists per **Run** in the Fuzzer
section.