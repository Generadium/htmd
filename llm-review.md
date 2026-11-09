# LLM review — Fuzzer section

Prompting Grok 4.6 with `Review the section named "Fuzzer" in translation_approach.md` produced the following output; strikethrough indicates LLM output that I deemed incorrect or irrelevant.

The LLM must not edit the spec in the same step as the review. Spec fixes from applied findings are in `translation_approach.md` (`### Fuzzer`).

## Rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1     | 8        | Default run is Faithful (`builder` + `Options`), not `htmd::convert` / Pure; timeout is a thread or child of the test binary, not the CLI; seed HTML literals from `tests/*.rs`, not the Rust source; structured stream must not nest `<a>` in `<a>`; corpus table Oracle column wins over stream class; priority 2 records that `convert` currently never returns `Err`; priority 4 compares html5ever DOMs, does not strcmp `round_trip` source; stream 2 points at the generator table, not an “HTML error list” |

Compile/run (`cargo build` / `cargo test` and the prompt *Verify that you can compile and run the project in the current git repo.*) is a later step, after further rounds.

## Round 1

Numbered findings; only issues where two implementers of this section would diverge.

1. Default API vs oracle mode. The section allowed `htmd::convert(&str)` or the builder and said “default options unless a case names Faithful or Pure.” `Options::default()` is `TranslationMode::Pure`. Priorities 3–4 judge Faithful Markdown and `tests/common/mod.rs` is Faithful. One implementer fuzzes Pure, another Faithful.

2. Subprocess vs timeout. “Do not go through … a subprocess other than the test binary” vs “Call `convert` in a thread or subprocess.” One implementer refuses a child process; another shells out to the `htmd` CLI.

3. Corpus seed `tests/*.rs`. That path is Rust test source. One implementer feeds `.rs` files to `convert`; another extracts HTML string literals.

4. Nested `<a>`. Stream 1 may nest `a` to depth 8. The corpus table says nested anchors are priority 1–2 only.

5. Section title “HTML the broken stream must include” lists Stream 1 payloads (`""`, math span, `&amp;`, `<p>3 < 4</p>`). Stream class and the Oracle column disagree.

6. Priority 4 cites `tests/common/mod.rs` then “parse as a DOM.” `round_trip` returns HTML source; several tests strcmp that string. One oracle strcmp’s, another compares html5ever trees.

7. Priority 2 treats `convert` `Err` as a bug except an “unrepresentable” `std::io::Error`. `html_to_tree` always returns `Ok`, so `convert` does not return `Err`, and this document names no expected `io::Error`.

8. After the edits, stream 2 still said “the HTML error list below.” The table is mixed Stream 1 / 2 payloads. Point at the generator table.

~~9. Cap inputs at 64 KiB and also require a 10,000-`a` attribute. Drop the huge-attribute row.~~ The row is under 64 KiB. It only restates the cap.

~~10. Specify a 50/50 mix of stream 1 and stream 2, and a second HTML parser as a validator of stream 2.~~ Mixing is already required. A second parser is a listed non-goal.

~~11. Rewrite the whole Fuzzer section to match libFuzzer / cargo-fuzz layout, or add `cargo fuzz` / a `[[bin]]` target now.~~ The assignment is a strategy in this document. The Run section says the fuzzer is a later binary.

No remaining listed payload has two oracles once the table’s Oracle column wins.
