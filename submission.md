Project 2 submission (draft) — htmd fuzzer
===========================================

The specification for this project is the section `### Fuzzer` in
[`translation_approach.md`](translation_approach.md). Detailed per-round LLM
output is in [`llm-review.md`](llm-review.md).

### Contributions

* **Robert:** Created first draft, organized final report.
* **Yerassyl:** LLM rounds 2, 5, and 6 and edits.
* **Alinur:** Completed first draft, LLM reviews 3 and 7 and edits.
* **Kurmanbek:** 1st and 4th round of LLM review and edits.

### Compiler test

The following summarize LLM output on each team member's computer, based on the
prompt *Verify that you can compile and run the project in the current git repo.*

#### Robert

Prompted **Codex** with *Verify that you can compile and run the project in the
current git repo.* Verified successfully with Rust **1.93.1**: `cargo build`
passed; `cargo clippy -- --deny warnings` passed; `cargo test` — 279 passed, 1
ignored; `cargo run --example simple` passed; `cargo run --example table`
produced the expected Markdown table. No files changed; the working tree is
clean.

#### Yerassyl

Prompted **Gemini 2.5 Flash** with *Verify that you can compile and run the
project in the current git repo.* The model said this is a normal Rust library
crate and that `cargo build` and `cargo test` from the repository root are the
right check, matching the Fuzzer **Run** section and CI. Verified on
**Generadium/htmd** with Rust **1.99.1** (`x86_64-pc-windows-gnu`):
`cargo build` — finished successfully; `cargo test` — **279** passed, **1**
ignored (unit, integration, and doc tests). Working tree clean after spec-only
edits.

#### Alinur

Prompted **Claude Sonnet 4.6** with the same prompt. The model confirmed
`Cargo.toml`, `src/`, and `tests/`, and recommended `cargo build` plus
`cargo test` at the crate root. Verified on **Generadium/htmd** with Rust
**1.93.1**: `cargo build` — finished successfully; `cargo test` — **279**
passed, **1** ignored. No build failures; fuzzer work is documentation-only so
far.

#### Kurmanbek

Prompted **Grok 4.6** with the same prompt. The model stated that `cargo test`
compiles and runs all test targets and `cargo build` compiles the library in the
default profile. Verified on **Generadium/htmd** with Rust **1.93.1**:
`cargo build` — passed; `cargo test` — **279** passed, **1** ignored. No
source changes required beyond `translation_approach.md` and review docs.

### Spec review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 8 | Default run is Faithful (`builder` + `Options`), not `htmd::convert` / Pure; timeout is a thread or child of the test binary, not the CLI; seed HTML literals from `tests/*.rs`, not the Rust source; structured stream must not nest `<a>` in `<a>`; corpus table Oracle column wins over stream class; priority 2 records that `convert` currently never returns `Err`; priority 4 compares html5ever DOMs, does not strcmp `round_trip` source; stream 2 points at the generator table, not an “HTML error list”. |
| 2 | 4 | Structured text nodes are serialized with HTML escaping; priority 4 compares parsed fragments under one context and ignores parser-inserted wrappers; empty and whitespace payloads explicitly treat `Err` as a finding; Comment / CDATA row points to this document's specification tables, not the generator table itself. |
| 3 | 4 | Mutations are string-level (not DOM-level); fragment context element named as `<div>`; finding storage changed to `#[ignore]` test (reconciles "keep passing" with "failing test case"); `&` double-escaping exception reworded to remove ambiguous "from the table below" phrase. |
| 4 | 4 | Stream 1 generator-table payloads stay verbatim (raw `<` in `"<p>3 < 4</p>"`); stream 1 root must parse under `<div>` (no bare `td`/`tr`/`th`/`li`); tag-delimited substring defined, skip duplicate/swap on unmatched tags; Pure run only for spec-table rows with a Pure expected column. |
| 5 | 4 | Stream class at oracle time via structured grammar, not mutation provenance; priority 3 only on specification tables with Expected, not generator-table rows alone; priority 4 DOM equivalence defined; corpus seeds limited to those specification tables. |
| 6 | 2 | Void/self-closing tags in tag-delimited mutations; CI time cap is wall-clock for the whole fuzz job. |
| 7 | 3 | Findings below, all applied. |

### Last LLM review

Model: **GPT-5.5**. Prompt: `Review the section named "Fuzzer" in
translation_approach.md`. Strikethrough indicates LLM output deemed incorrect or
irrelevant.

Numbered findings; only issues where two implementers of this section would
diverge.

*(None after rounds 1–6.)*

**Review text.** Generator (two streams, mutations, generator table), oracle
(stream class at check time, priorities 1–5), and Run (`cargo test` /
`cargo build`, `#[ignore]` findings) read as one strategy. Stream 1 vs 2,
Faithful default vs Pure for spec-table rows, fragment parsing under `<div>`, and
specification-table seeds are aligned across the section. I did not find a place
where two implementers would still pick different APIs, oracles, or run commands.

~~1. Add a second HTML parser to validate stream 2.~~ Listed non-goal.

~~2. Remove the 64 KiB cap as unrealistic.~~ The cap is explicit and the
huge-attribute row exercises it.

~~3. Require every contributor to use the same LLM for all rounds.~~ Each round
records model and prompt; mixed models are fine.
