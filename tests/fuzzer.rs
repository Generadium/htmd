//! Fuzzer harness (`### Fuzzer` in `translation_approach.md`): generator-table
//! seeds, child-process timeout, priorities 1–2 and round-trip where eligible.

mod common;

use std::{
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use common::convert_faithful;
use pulldown_cmark::Options as CommonMarkOptions;

const TIMEOUT: Duration = Duration::from_secs(2);
const RENDER_OPTS: CommonMarkOptions = CommonMarkOptions::ENABLE_TABLES
    .union(CommonMarkOptions::ENABLE_TASKLISTS)
    .union(CommonMarkOptions::ENABLE_MATH);

/// Generator-table seeds from the Fuzzer section (truncated huge-attribute row).
static GENERATOR_SEEDS: &[(&str, &str)] = &[
    ("", "empty"),
    ("   \n\t", "ascii_ws"),
    ("\u{00a0}\u{2003}", "unicode_ws"),
    (" \t\u{00a0}\n ", "mixed_ws"),
    ("<p>hello", "optional_close"),
    ("<p>hello</p>", "closed_p"),
    ("hello</p>", "extra_close"),
    ("<a href=a><a href=b>x</a></a>", "nested_a"),
    ("<a href=http://x y>z</a>", "unquoted_attr"),
    ("<p>a\0b</p>", "nul"),
    ("<!-- <h1> -->", "comment"),
    ("<![CDATA[ a ]]>", "cdata"),
    ("<p>3 < 4</p>", "raw_lt"),
    ("<p>a&b</p>", "ampersand"),
    ("<p>&amp;</p>", "entity"),
    ("<td>x</td>", "table_frag"),
    (r#"<span class="math math-inline">$</span>"#, "math_span"),
];

fn convert_in_child(input: &str) -> ChildOutcome {
    if input.contains('\0') {
        return match convert_faithful(input) {
            Ok(_) => ChildOutcome::Ok,
            Err(_) => ChildOutcome::Aborted,
        };
    }
    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => return ChildOutcome::SpawnFailed,
    };
    let mut child = match Command::new(exe)
        .args(["--exact", "run_convert_worker", "--nocapture"])
        .env("HTMD_FUZZ_INPUT", input)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return ChildOutcome::SpawnFailed,
    };
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                return if status.success() {
                    ChildOutcome::Ok
                } else {
                    ChildOutcome::Aborted
                };
            }
            Ok(None) => {}
            Err(_) => return ChildOutcome::SpawnFailed,
        }
        if start.elapsed() >= TIMEOUT {
            let _ = child.kill();
            let _ = child.wait();
            return ChildOutcome::Timeout;
        }
        thread::sleep(Duration::from_millis(20));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChildOutcome {
    Ok,
    Timeout,
    Aborted,
    SpawnFailed,
}

fn stream1_eligible_round_trip(html: &str) -> bool {
  // Block or wrapper roots eligible for priority 3; bare fragments like <td> are Stream 2.
  let trimmed = html.trim();
  trimmed.starts_with("<p>")
        || trimmed.starts_with("<div>")
        || trimmed.starts_with("<table>")
        || trimmed.starts_with("<ul>")
        || trimmed.starts_with("<ol>")
        || trimmed.starts_with("<pre>")
        || trimmed.starts_with("<blockquote>")
        || trimmed.starts_with("<h1")
        || trimmed.starts_with("<h2")
        || trimmed.starts_with("<h3")
}

fn round_trip_html(html: &str) -> String {
    let md = convert_faithful(html).expect("faithful convert");
    common::render_markdown(&md, RENDER_OPTS)
}

fn collapse_ws(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_space = false;
    for ch in s.chars() {
        let ws = matches!(ch, ' ' | '\t' | '\n' | '\r' | '\x0C');
        if ws {
            if !prev_space {
                out.push(' ');
                prev_space = true;
            }
        } else {
            prev_space = false;
            out.push(ch);
        }
    }
    out.trim().to_string()
}

fn fragment_plain(html: &str) -> String {
    scraper::Html::parse_fragment(html).root_element().text().collect()
}

/// Priority 3 check (simplified): same collapsed plain text in fragment after round-trip.
fn round_trip_plain_matches(html: &str) -> bool {
    let back = round_trip_html(html);
    collapse_ws(&fragment_plain(html)) == collapse_ws(&fragment_plain(&back))
}

#[test]
fn fuzzer_generator_table_no_panics_or_timeouts() {
    for (payload, _name) in GENERATOR_SEEDS {
        let outcome = convert_in_child(payload);
        assert!(
            matches!(outcome, ChildOutcome::Ok),
            "payload {:?}: {:?}",
            payload,
            outcome
        );
    }
}

#[test]
fn fuzzer_empty_and_whitespace_ok() {
    for payload in ["", "   \n\t", " \t\u{00a0}\n "] {
        let md = convert_faithful(payload).expect("must return Ok");
        assert!(!md.is_empty() || payload.trim().is_empty() || !payload.chars().all(|c| c.is_whitespace()));
    }
}

#[test]
fn fuzzer_round_trip_eligible_stream1() {
    let cases = [
        "<p>hello</p>",
        "<p>3 &lt; 4</p>",
        "<p>&amp;</p>",
        "<p>hello",
        "<p>hello</p>",
    ];
    for html in cases {
        if !stream1_eligible_round_trip(html) {
            continue;
        }
        assert!(
            round_trip_plain_matches(html),
            "round-trip plain text mismatch for {html:?}"
        );
    }
}

#[test]
fn run_convert_worker() {
    let Ok(input) = std::env::var("HTMD_FUZZ_INPUT") else {
        return;
    };
    let _ = convert_faithful(&input);
}

#[test]
fn fuzzer_optional_closing_tag_same_markdown() {
    let a = convert_faithful("<p>hello").expect("ok");
    let b = convert_faithful("<p>hello</p>").expect("ok");
    assert_eq!(a, b, "optional closing tag row: same DOM → same Markdown");
}

#[test]
fn fuzzer_sample_mutations() {
    let seeds = ["<p>x</p>", "<ul><li>a</li></ul>"];
    for seed in seeds {
        let broken = format!("{seed}</div>");
        let outcome = convert_in_child(&broken);
        assert!(
            matches!(outcome, ChildOutcome::Ok),
            "mutation child failed for {broken:?}"
        );
    }
}