# LLM review — Fuzzer section

Each round prompted an LLM with `Review the section named "Fuzzer" in translation_approach.md`; strikethrough indicates LLM output that I deemed incorrect or irrelevant. Round 1 used Grok 4.6; Round 2 used GPT-5.5; Round 3 used Claude Sonnet 4.6; Round 4 used Grok 4.6; Round 5 used Gemini 2.5 Flash; Round 6 used Claude Sonnet 4.6; Round 7 used GPT-5.5; Round 8–9 used Codex (Robert, after Jones); Round 10 used GPT-5.5.

The LLM must not edit the spec in the same step as the review. Spec fixes from applied findings are in `translation_approach.md` (`### Fuzzer`).

## Rounds


| Round                      | Findings | Main changes made                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| -------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1 (used Grok 4.6)          | 8        | Default run is Faithful (`builder` + `Options`), not `htmd::convert` / Pure; timeout is a thread or child of the test binary, not the CLI; seed HTML literals from `tests/*.rs`, not the Rust source; structured stream must not nest `<a>` in `<a>`; corpus table Oracle column wins over stream class; priority 2 records that `convert` currently never returns `Err`; priority 4 compares html5ever DOMs, does not strcmp `round_trip` source; stream 2 points at the generator table, not an “HTML error list” |
| 2 (used GPT-5.5)           | 4        | Structured text nodes are serialized with HTML escaping; priority 4 compares parsed fragments under one context and ignores parser-inserted wrappers; empty and whitespace payloads explicitly treat `Err` as a finding; Comment / CDATA row points to this document's specification tables, not the generator table itself                                                                                                                                                                                         |
| 3 (used Claude Sonnet 4.6) | 4        | Mutations are string-level (not DOM-level); fragment context element named as `<div>`; finding storage changed to `#[ignore]` test (reconciles "keep passing" with "failing test case"); `&` double-escaping exception reworded to remove ambiguous "from the table below" phrase                                                                                                                                                                                                                                   |
| 4 (used Grok 4.6)          | 4        | Stream 1 generator-table payloads stay verbatim (raw `<` in `"<p>3 < 4</p>"`); stream 1 root must parse under `<div>` (no bare `td`/`tr`/`th`/`li`); tag-delimited substring defined, skip duplicate/swap on unmatched tags; Pure run only for spec-table rows with a Pure expected column |
| 5 (used Gemini 2.5 Flash)  | 4        | Stream class at oracle time via structured grammar, not mutation provenance; priority 3 only on specification tables with Expected, not generator-table rows alone; priority 4 DOM equivalence defined; corpus seeds limited to those specification tables |
| 6 (used Claude Sonnet 4.6) | 2        | Void/self-closing tags in tag-delimited mutations; CI time cap is wall-clock for the whole fuzz job |
| 8 (used Codex 6.1-Sol Extra High)            | 4        | Bare root text and phrasing content receive only priorities 1–2, with separate paragraph-wrapped variants for round-trip checks; whitespace comparison rules added; timeout consistently requires a child process; specification-table expected outputs delegated to deterministic tests. Whitespace details and generator-table consistency edits remain pending. |
| 9 (used Codex 6.1-Sol Extra High)            | 4  | Deletion mutations use substrings with endpoints at UTF-8 character boundaries; rendering explicitly enables tables, task lists, and math; the whitespace set and locations where formatting whitespace may be ignored are specified; Unicode whitespace, literal content, and attribute values are preserved. Unapplied recommendations are crossed out in the Round 9 description. |
| 10 (used GPT-5.5)           | 3        | Priority 3 references Oracle steps 2–4 instead of duplicating rules; whitespace collapse between inline elements; Comment/CDATA and Math span rows aligned with deterministic tests; typo fixes in mutations |


Compile/run evidence per team member is in [`submission.md`](submission.md) (Project 2 draft).

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

1. Structured text containing `<` and `&`. Stream 1 says text nodes come from a pool including `<` and `&`, and also says every open tag is closed and the stream may be checked as valid HTML. One implementer escapes text nodes as `&lt;` / `&amp;`; another writes raw `<` / `&` and creates broken HTML inside stream 1.
2. Priority 4 DOM comparison context. The section says parse the input HTML and round-trip HTML with `html5ever`, but the fuzzer input is a fragment. One implementer compares full parser documents with inserted `<html>` / `<body>` wrappers; another compares fragment children under a fixed context. Wrapper insertion can become the finding instead of an htmd mismatch.
3. Empty and whitespace-only rows. The table says “Must return `Ok`, not panic,” while the oracle already says any `Err` is a priority 2 finding. One implementer records `Err` for these rows as a finding; another reads the row as only forbidding panic.
4. Comment / CDATA expected-output escape hatch. The row says “if treated as a table row in this file,” but the row itself is in the generator table. One implementer applies only priorities 1–2 to the listed payloads; another looks for expected Markdown in this document's specification tables.

~~5. Require a fixed random seed and publish the seed value in the spec.~~ Reproducibility is useful for an implementation, but this strategy document only needs saved failing inputs; the Run section already says a finding becomes a failing test case.

All four findings applied.

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

## Round 5

Model: Gemini 2.5 Flash. Prompt: `Review the section named "Fuzzer" in translation_approach.md`. Strikethrough indicates LLM output deemed incorrect or irrelevant.

Numbered findings; only issues where two implementers of this section would diverge.

1. Stream class after mutation. Stream 2 mutates stream 1. The oracle branches on “Stream 1” vs “Stream 2” but never says whether class follows how the input was produced or how it reads at check time. One implementer tags every mutation output as stream 2; another reclassifies mutated strings that still match the structured grammar as stream 1.
2. Priority 3 vs generator table. Priority 3 names “a row in this document's tables,” and the generator table is in the same file. Rows such as Math span say “expected follows the Math section” without an Expected cell beside them. One implementer runs priority 3 on those rows; another applies only priorities 1–2 unless the payload also appears in a specification table with Expected output.
3. Priority 4 equivalence. “Same elements and text” does not say whether attributes matter, whether text is normalized, or how to compare implied structure. One implementer strcmp’s serialized HTML; another walks trees with ad hoc rules.
4. Corpus seed tables. “HTML-in / Faithful-expected tables in this file” could mean every Markdown table in `translation_approach.md`. One implementer loads hundreds of incidental tables; another only feature sections that pair HTML input with Expected Markdown.

~~5. Require exactly 50% stream 1 / stream 2.~~ Mixing is already required. A fixed ratio is extra.

All four findings applied.

## Round 6

Model: Claude Sonnet 4.6. Prompt: `Review the section named "Fuzzer" in translation_approach.md`. Strikethrough indicates LLM output deemed incorrect or irrelevant.

Numbered findings; only issues where two implementers of this section would diverge.

1. Tag-delimited substring on void tags. The mutation definition requires an opening tag through its matching closer. For `br`, `hr`, and `img`, one implementer treats the opening tag alone as the substring; another waits for a `</br>` that never comes and skips all duplicate/swap mutations on common structured seeds.
2. Five-minute CI cap. “A shorter CI job may cap time instead of count (five minutes, then stop)” does not say whether five minutes is the whole fuzz job or five minutes per input. One implementer stops the entire run at 5:00; another times out each case at five minutes and never reaches 10,000 cases.

~~3. Mandate libFuzzer now.~~ The Run section still defers the binary.

Both findings applied.

## Final review (Round 7)

Model: GPT-5.5. Prompt: `Review the section named "Fuzzer" in translation_approach.md`. Strikethrough indicates LLM output deemed incorrect or irrelevant.

Numbered findings; only issues where two implementers of this section would diverge.

*(None after rounds 1–6.)*

**Review text.** Generator (two streams, mutations, generator table), oracle (stream class at check time, priorities 1–5), and Run (`cargo test` / `cargo build`, `#[ignore]` findings) read as one strategy. Stream 1 vs 2, Faithful default vs Pure for spec-table rows, fragment parsing under `<div>`, and specification-table seeds are aligned across the section. I did not find a place where two implementers would still pick different APIs, oracles, or run commands.

~~1. Add a second HTML parser to validate stream 2.~~ Listed non-goal.

~~2. Remove the 64 KiB cap as unrealistic.~~ The cap is explicit and the huge-attribute row exercises it.

~~3. Require every contributor to use the same LLM for all rounds.~~ Each round records model and prompt; mixed models are fine.

## Round 8

Reviewer: Codex. Prompt: `Review the section named "Fuzzer" in translation_approach.md`. Follow-up: `How should these suggestions be implemented? Do not edit the file.`

This entry records only this session's four findings, implementation guidance, and revisions subsequently visible in `translation_approach.md`. The review and guidance did not edit the specification; this documentation step changes only `llm-review.md`.

Numbered findings:

1. DOM comparison rejects valid inline roots. Stream 1 permits `<em>x</em>`, which translates to `*x*` and renders as `<p><em>x</em></p>`. The added paragraph is not a parser-inserted document wrapper, so the strict comparison reports a valid translation as a finding.
2. Whitespace equivalence is undefined. The comparator permits unspecified whitespace differences, while rendering even `<p>x</p>` adds a trailing newline and therefore another fragment text node. Implementers could either report valid output or normalize away significant spacing.
3. Timeout instructions conflict. The introduction permits a thread or child process, but the Oracle requires a terminable child process. A thread timeout leaves the conversion running and cannot isolate a process abort.
4. Specification-table expected-output exceptions conflict with the priority table. The Stream 2 paragraph implies extra checks for specification-table matches, while priority 4 limits Stream 2 to conversion failures and the following paragraph delegates examples to deterministic tests.

Main changes made to the specification during this session:

1. Round-trip eligibility now excludes Stream 1 inputs with text or phrasing content directly at the fragment root. These receive priorities 1–2; separate paragraph-wrapped variants provide round-trip coverage. Corpus seeds remain verbatim.
2. The comparison rules now mention ignoring ASCII-whitespace-only text nodes, collapsing ordinary whitespace, and preserving separating whitespace and literal text. This begins implementing the proposed context-sensitive normalization; the remaining details are listed below.
3. The introduction now requires a child process of the test binary. The parent enforces a 2-second deadline, terminates and waits for a timed-out child, records the failing input, and continues. This agrees with the Oracle's process requirement.
4. The Stream 2 paragraph now delegates specification-table expected outputs to deterministic tests. Only explicit additional checks in the generator table override the usual priorities 1–2.

Remaining follow-ups from this round's guidance:

* Scope ignored whitespace-only nodes to fragment boundaries and gaps between block children in block-only containers. Specify tab, LF, form feed, CR, and space; preserve Unicode whitespace and significant inline separators; apply documented emphasis-edge whitespace movement. Clarify preservation of attribute values (the current wording says "absolute values").
* Make priority 3 reference the authoritative comparison rules instead of duplicating them. The table still refers to rules "below" although they appear above it.
* Remove the Comment / CDATA row's specification-table expected-output exception and replace the Math span row's "expected follows the Math section" with explicit applicable checks. Retain the optional-closing-tag row's paired Markdown-equivalence check.

Status: Revisions partially applied; the follow-ups above remain. Validation was a static review of the specification, converter, and existing tests. Cargo was unavailable, so no build, Clippy, or test run was performed. No converter or fuzzer implementation changed in this session.

## Round 9

Reviewer: Codex. Scope: document this session's specification revisions and the disposition of the Round 8 recommendations, using the latest saved version of `translation_approach.md`. This follows up on the four findings already recorded in Round 8 and includes the saved mutation-safety clarification. Earlier rounds are retained as historical records.

Findings and disposition:

1. Inline-root DOM comparison. The Round 8 restriction remains: bare root text and phrasing content receive priorities 1–2, with separate paragraph-wrapped variants for round-trip checks. No additional change was made for this finding in Round 9.
2. Whitespace equivalence. The revised comparator now specifies tab, LF, form feed, CR, and space as collapsible whitespace. Ignored formatting whitespace is restricted to fragment boundaries and gaps between block children in block-only containers. Unicode whitespace, literal content in `pre`, `code`, and math, and attribute values are explicitly preserved.
3. Timeout isolation. The child-process requirement, 2-second deadline, and termination and waiting behavior recorded in Round 8 remain in place. No additional timeout change was made in Round 9.
4. Expected-output checks. The Stream 2 paragraph continues to delegate specification-table expected outputs to deterministic tests. The recommended changes to the Comment / CDATA and Math span generator-table rows were not applied; their existing wording remains.

Main changes made since the Round 8 entry during this session:

* Deletion mutations now remove a substring whose endpoints are UTF-8 character boundaries, replacing the earlier "delete a byte slice" instruction. This preserves valid UTF-8 when mutating generated inputs containing Unicode characters.
* Markdown rendering now explicitly passes `CommonMarkOptions::ENABLE_TABLES`, `CommonMarkOptions::ENABLE_TASKLISTS`, and `CommonMarkOptions::ENABLE_MATH` to `render_markdown`.
* The whitespace comparison text now names the five collapsible characters and limits ignored formatting whitespace to the structural locations described above.
* Preservation of Unicode whitespace, literal content, and attribute values is explicit; the earlier "absolute values" wording has been replaced with "attribute values".

Recommendations ignored in this revision are crossed out below. Here, strikethrough records that a recommendation was not applied; no reason for omitting it was provided.

~~5. Specify ordinary-text whitespace collapsing and block-boundary trimming, preserve inline separator spaces explicitly, and apply documented emphasis-edge whitespace movement.~~ The current bullets define the whitespace set and formatting-whitespace locations but do not state these remaining normalization rules explicitly.

~~6. Make priority 3 reference the authoritative comparison rules instead of duplicating them, and correct its reference from "below" to "above".~~ Priority 3 retains the duplicated instructions and the reference to rules "below".

~~7. Remove the Comment / CDATA row's specification-table expected-output exception and replace the Math span row's "expected follows the Math section" with explicit applicable checks.~~ Both rows retain their existing wording. The optional-closing-tag row's paired Markdown-equivalence check remains in place.

Validation: Reread the latest saved specification and compared it with this session's Round 8 record and the previous Round 9 summary. The three crossed-out recommendations remain unapplied in that saved version. This documentation step changes only `llm-review.md`; no converter, fuzzer, or specification edit was made by the assistant. No build, Clippy, or test run was performed for this documentation update.

## Round 10

Model: GPT-5.5. Prompt: `Review the section named "Fuzzer" in translation_approach.md` after Jones requested changes and Codex rounds 8–9.

Numbered findings; only issues where two implementers of this section would diverge.

1. Priority 3 duplicated Oracle steps 2–4 and said “rules below” while the comparison rules appear above the priority table. One implementer re-implements a second DOM walk; another calls the shared procedure once.
2. Round 9 left Comment/CDATA and Math span generator rows implying specification-table expected output inside the fuzz oracle, conflicting with Stream 2 delegation to deterministic tests.
3. Whitespace comparison named collapsible characters but did not say whether runs collapse inside text nodes or only at block boundaries.

~~4. Remove round-trip entirely for Stream 1.~~ Round-trip remains for eligible wrapped roots; bare phrasing roots stay at priorities 1–2.

All three findings applied in `translation_approach.md`.

## Final review (post–Round 10)

Model: GPT-5.5. Prompt: same as Round 10.

*(No numbered divergence findings after Round 10.)*

**Review text.** Child-process timeout, Stream 1 vs 2 at check time, priorities 1–4, generator table Oracle column, and deterministic specification examples are aligned. Ready for another instructor pass on [#7](https://github.com/bjones1/htmd/pull/7).
