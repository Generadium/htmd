# LLM review — Fuzzer section

Each round prompted an LLM with `Review the section named "Fuzzer" in translation_approach.md`; strikethrough indicates LLM output that I deemed incorrect or irrelevant. Round 1 used Grok 4.6; Round 2 used GPT-5.5; Round 3 used Claude Sonnet 4.6; Round 4 used Grok 4.6.

The LLM must not edit the spec in the same step as the review. Spec fixes from applied findings are in `translation_approach.md` (`### Fuzzer`).

## Rounds


| Round                      | Findings | Main changes made                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| -------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1 (used Grok 4.6)          | 8        | Default run is Faithful (`builder` + `Options`), not `htmd::convert` / Pure; timeout is a thread or child of the test binary, not the CLI; seed HTML literals from `tests/*.rs`, not the Rust source; structured stream must not nest `<a>` in `<a>`; corpus table Oracle column wins over stream class; priority 2 records that `convert` currently never returns `Err`; priority 4 compares html5ever DOMs, does not strcmp `round_trip` source; stream 2 points at the generator table, not an “HTML error list” |
| 2 (used GPT-5.5)           | 4        | Structured text nodes are serialized with HTML escaping; priority 4 compares parsed fragments under one context and ignores parser-inserted wrappers; empty and whitespace payloads explicitly treat `Err` as a finding; Comment / CDATA row points to this document's specification tables, not the generator table itself                                                                                                                                                                                         |
| 3 (used Claude Sonnet 4.6) | 4        | Mutations are string-level (not DOM-level); fragment context element named as `<div>`; finding storage changed to `#[ignore]` test (reconciles "keep passing" with "failing test case"); `&` double-escaping exception reworded to remove ambiguous "from the table below" phrase                                                                                                                                                                                                                                   |
| 4 (used Grok 4.6)          | 4        | Stream 1 generator-table payloads stay verbatim (raw `<` in `"<p>3 < 4</p>"`); stream 1 root must parse under `<div>` (no bare `td`/`tr`/`th`/`li`); tag-delimited substring defined, skip duplicate/swap on unmatched tags; Pure run only for spec-table rows with a Pure expected column |


Compile/run (`cargo build` / `cargo test` and the prompt *Verify that you can compile and run the project in the current git repo.*) is a later step, after further rounds.

## Round 1

Numbered findings; only issues where two implementers of this section would diverge.

1. Default API vs oracle mode. The section allowed `htmd::convert(&str)` or the builder and said “default options unless a case names Faithful or Pure.” `Options::default()` is `TranslationMode::Pure`. Priorities 3–4 judge Faithful Markdown and `tests/common/mod.rs` is Faithful. One implementer fuzzes Pure, another Faithful.
2. Subprocess vs timeout. “Do not go through … a subprocess other than the test binary” vs “Call `convert` in a thread or subprocess.” One implementer refuses a child process; another shells out to the `htmd` CLI.
3. Corpus seed `tests/*.rs`. That path is Rust test source. One implementer feeds `.rs` files to `convert`; another extracts HTML string literals.
4. Nested `<a>`. Stream 1 may nest `a` to depth 8. The corpus table says nested anchors are priority 1–2 only.
5. Section title “HTML the broken stream must include” lists Stream 1 payloads (`""`, math span, `&`, `<p>3 < 4</p>`). Stream class and the Oracle column disagree.
6. Priority 4 cites `tests/common/mod.rs` then “parse as a DOM.” `round_trip` returns HTML source; several tests strcmp that string. One oracle strcmp’s, another compares html5ever trees.
7. Priority 2 treats `convert` `Err` as a bug except an “unrepresentable” `std::io::Error`. `html_to_tree` always returns `Ok`, so `convert` does not return `Err`, and this document names no expected `io::Error`.
8. After the edits, stream 2 still said “the HTML error list below.” The table is mixed Stream 1 / 2 payloads. Point at the generator table.

~~9. Cap inputs at 64 KiB and also require a 10,000-~~`a` ~~attribute. Drop the huge-attribute row.~~ The row is under 64 KiB. It only restates the cap.

~~10. Specify a 50/50 mix of stream 1 and stream 2, and a second HTML parser as a validator of stream 2.~~ Mixing is already required. A second parser is a listed non-goal.

~~11. Rewrite the whole Fuzzer section to match libFuzzer / cargo-fuzz layout, or add~~ `cargo fuzz` ~~/ a~~ `[[bin]]` ~~target now.~~ The assignment is a strategy in this document. The Run section says the fuzzer is a later binary.

No remaining listed payload has two oracles once the table’s Oracle column wins.

## Round 2

Model: GPT-5.5. Prompt: `Review the section named "Fuzzer" in translation_approach.md`.

Numbered findings; only issues where two implementers of this section would diverge.

1. Structured text containing `<` and `&`. Stream 1 says text nodes come from a pool including `<` and `&`, and also says every open tag is closed and the stream may be checked as valid HTML. One implementer escapes text nodes as `<` / `&`; another writes raw `<` / `&` and creates broken HTML inside stream 1.
2. Priority 4 DOM comparison context. The section says parse the input HTML and round-trip HTML with `html5ever`, but the fuzzer input is a fragment. One implementer compares full parser documents with inserted `<html>` / `<body>` wrappers; another compares fragment children under a fixed context. Wrapper insertion can become the finding instead of an htmd mismatch.
3. Empty and whitespace-only rows. The table says “Must return `Ok`, not panic,” while the oracle already says any `Err` is a priority 2 finding. One implementer records `Err` for these rows as a finding; another reads the row as only forbidding panic.
4. Comment / CDATA expected-output escape hatch. The row says “if treated as a table row in this file,” but the row itself is in the generator table. One implementer applies only priorities 1–2 to the listed payloads; another looks for expected Markdown in this document's specification tables.

~~5. Require a fixed random seed and publish the seed value in the spec.~~ Reproducibility is useful for an implementation, but this strategy document only needs saved failing inputs; the Run section already says a finding becomes a failing test case.

## Round 3

Model: Claude Sonnet 4.6. Prompt: `Review the section named "Fuzzer" in translation_approach.md`. Strikethrough indicates LLM output deemed incorrect or irrelevant.

Numbered findings; only issues where two implementers of this section would diverge.

1. Mutation level: string vs DOM. "Delete a slice" and "replace a named character reference" are string operations. "Duplicate a subtree" and "swap two siblings" are DOM operations. One implementer builds and mutates a parsed DOM tree; another works on the string directly. They produce different mutation spaces and different test inputs.
2. Fragment context element not named. Priority 4 said "parse as fragments under the same context with html5ever." `html5ever` fragment parsing requires a context element; parsing `<td>x</td>` under `<div>` vs `<table>` yields different DOMs. One implementer uses `<div>`; another uses `<body>`. The context element was never specified.
3. `cargo test` passes vs. findings are failing test cases. "A fuzzer finding is a failing test case added next to the existing tables" and "`cargo test` must keep passing" directly contradict. One implementer adds active failing tests (CI breaks); another marks them `#[ignore]` (CI passes). The distinction matters for whether the repo CI stays green.
4. `&` double-escaping exception. "Serialize text nodes with HTML escaping … except when the case intentionally emits a named character reference from the table below." The generator table has no column named "named character reference." One implementer reads "from the table below" as only the `&` Ampersand row; another searches the whole table for any named reference. The exception was narrower than phrased.

~~5. Remove the~~ `Huge attribute` ~~row: the generator already caps at 64 KiB, making the row redundant.~~ The row makes the cap concrete with a specific shape of input (long attribute vs long text body). Removing it leaves that shape untested.

~~6. Add~~ `cargo fuzz` ~~/~~ `cargo-fuzz` ~~as the required runner in the Run section.~~ The section already says the fuzzer is "a later binary or `cargo test` target." Naming a framework now over-specifies the implementation.

All four findings applied.

## Round 4

Model: Grok 4.6. Prompt: `Review the section named "Fuzzer" in translation_approach.md`. Strikethrough indicates LLM output deemed incorrect or irrelevant.

Numbered findings; only issues where two implementers of this section would diverge.

1. Structured escaping vs generator-table Stream 1. Stream 1 serializes text with `<` → `&lt;`, but the table lists `"<p>3 < 4</p>"` as Stream 1. One implementer never emits raw `<` in stream 1; another emits the table payload verbatim. The table's Oracle column is supposed to win, but the escaping rule did not say so.
2. Stream 1 root vs `<div>` fragment context. Priority 4 parses both sides under `<div>`. Stream 1 may still emit a bare `td`/`tr`/`li` as the fragment root (those tags are in the element list). html5ever under `<div>` does not keep a `td` as `td`. One implementer uses those roots and flags wrapper/identity changes; another never emits them in stream 1 (the generator table already puts `"<td>x</td>"` in priorities 1–2).
3. “Tag-delimited substring” was undefined. Round 3 locked mutations to strings (`duplicate a tag-delimited substring`, `swap two adjacent tag-delimited sibling substrings`) without saying what the delimiter is. One implementer splits on every `<…>`; another requires a matching closer. Unmatched stream 2 seeds then diverge.
4. When Pure runs. The intro still said use Pure “when a case names Pure,” but the generator table has no mode column. Specification tables elsewhere do have a Pure expected column. One implementer never runs Pure; another runs Pure on every case.

~~5. Require a 50/50 mix of stream 1 and stream 2.~~ Mixing is already required. A ratio is extra.

~~6. Add `cargo fuzz` or a `[[bin]]` now.~~ The Run section still says the fuzzer is a later binary or `cargo test` target.

All four findings applied.