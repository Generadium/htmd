Project 2 submission — htmd fuzzer
===================================

Specification: `### Fuzzer` in [`translation_approach.md`](translation_approach.md).
Reviews: [`llm-review.md`](llm-review.md). Pull request:
[bjones1/htmd#7](https://github.com/bjones1/htmd/pull/7).

### Contributions

* **Robert:** First draft, Jones oracle revision, LLM rounds 8–9 (Codex), team coordination.
* **Yerassyl:** LLM rounds 2, 5, 6, 10; spec follow-ups after Jones; `submission.md`; fuzzer `tests/fuzzer.rs`; final Project 2 submission integration and push to `main`.
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
| 1 | 3 | Added `tests/fuzzer.rs`: generator-table seeds, child-process convert with 2s timeout, round-trip check for eligible Stream 1; NUL via in-process convert on Windows env limits; documented `cargo test --test fuzzer` in spec **Run** |
| 2 | 0 | Optional-closing-tag generator row test; `cargo test` / `cargo test --test fuzzer` green; submission and `llm-review` implementation section complete |

### Compiler test (implementation)

Prompt: *Review the fuzzer implementation in tests/fuzzer.rs against the Fuzzer section.*

#### Yerassyl

**GPT-5.5**, after adding `tests/fuzzer.rs`: `cargo test --test fuzzer` — 6 passed;
full `cargo test` — passed (Rust 1.99.1).

### Last LLM spec review

Model: **GPT-5.5**. Prompt: `Review the section named "Fuzzer" in
translation_approach.md`.

*(No divergence findings after Round 10; see **Final review (post–Round 10)** in
`llm-review.md`.)*

### Last LLM implementation review

Model: **GPT-5.5**. Prompt: `Review the fuzzer implementation in tests/fuzzer.rs
against the section named "Fuzzer" in translation_approach.md`.

1. Child process cannot pass NUL through the environment on Windows; one
   implementer skips child for `\0` payloads, another base64-encodes env. **Applied:**
   in-process convert for NUL only.
2. Round-trip comparison uses collapsed plain text, not full DOM equivalence from
   the spec. **Accepted** as first harness; DOM walk deferred.
3. Generator run uses fixed seeds, not 10,000 cases or mutations library. **Accepted**
   for CI smoke; volume stays behind `cargo test --test fuzzer` until libFuzzer.

~~4. Remove child-process requirement.~~ Spec still requires child for normal
payloads; harness spawns `run_convert_worker`.