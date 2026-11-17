Project 2 — final submission (htmd fuzzer)
===========================================

Specification: `### Fuzzer` in [`translation_approach.md`](translation_approach.md).
Reviews: [`llm-review.md`](llm-review.md). Pull request:
[bjones1/htmd#7](https://github.com/bjones1/htmd/pull/7) (same PR as draft).

Draft compiler-check notes remain in commit `418294b` and earlier `submission.md`
history.

### Contributions

* **Robert:** Led post–Jones spec revision (`raf322: review revisions`) and Codex
  rounds 8–9 in `llm-review.md`.
* **Yerassyl:** `tests/fuzzer.rs` and fuzzer implementation (`y_suleimenov:
  finalize fuzzer implementation and Project 2 submission`).
* **Alinur:** Spec round 10 after Jones (`al_kabylzhaparov: fuzzer spec round 10
  and submission update`); complete fuzzer spec and LLM round 7 (draft).
* **Kurmanbek:** LLM rounds 1 and 4 (draft); Project 2 final `submission.md`
  format and push to `main`.

### Final specification review rounds

*(Draft rounds 1–7 are in [`llm-review.md`](llm-review.md); not repeated here.)*

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 8 | 4 | After Jones CHANGES_REQUESTED: inline-root round-trip rules; child-process-only timeout; whitespace comparison draft; Stream 2 delegates spec-table expected output to deterministic tests (Robert / Codex) |
| 9 | 4 | UTF-8-safe deletion mutations; pulldown tables/tasklists/math; whitespace set and block-boundary rules; crossed-out unapplied Codex items recorded (Robert / Codex) |
| 10 | 3 | Priority 3 cross-ref to Oracle steps 2–4; Comment/CDATA and Math generator rows; whitespace collapse between inline elements (Yerassyl / GPT-5.5) |

### Implementation review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 3 | `tests/fuzzer.rs`: generator-table seeds, child `run_convert_worker` + 2s timeout, round-trip smoke for eligible Stream 1; NUL in-process on Windows |
| 2 | 0 | Optional-closing-tag row; six fuzzer tests green; disposition recorded below |

### Last LLM specification review

Model: **GPT-5.5**. Prompt: `Review the section named "Fuzzer" in
translation_approach.md` (post–Round 10).

Numbered findings from Round 10:

1. Priority 3 duplicated Oracle steps 2–4 and said “rules below” while comparison
   rules sit above the priority table. **Applied:** cross-reference to steps 2–4.
2. Comment/CDATA and Math span rows implied specification-table expected output in
   the fuzz oracle. **Applied:** aligned with deterministic tests / Stream 2.
3. Whitespace comparison did not state whether runs collapse inside text nodes.
   **Applied:** collapse rules between inline elements.

~~4. Remove round-trip entirely for Stream 1.~~ Round-trip remains for eligible
wrapped roots.

**Final review (same model and prompt):** *(No further numbered divergence
findings.)* Child-process timeout, Stream class at check time, priorities 1–4,
generator Oracle column, and delegation of specification examples to deterministic
tests are consistent in the saved spec.

### Last LLM implementation review

Model: **GPT-5.5**. Prompt: `Review the fuzzer implementation in tests/fuzzer.rs
against the section named "Fuzzer" in translation_approach.md` (Round 2).

*(No new divergence findings after optional-closing-tag coverage.)*

`cargo test --test fuzzer` — 6 passed; full `cargo test` passed (Rust 1.99.1).

Round 1 items (dispositioned for the harness scope):

1. NUL in environment on Windows → in-process convert for `\0` only.
2. Full DOM oracle in spec vs plain-text round-trip in tests → smoke harness;
   DOM walk deferred.
3. Fixed seeds vs 10k cases and mutation library → CI smoke; volume documented in
   spec **Run**.

~~4. Drop child-process requirement for all inputs.~~ Child process used for
non-NUL payloads per spec.