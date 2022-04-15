use anyhow::{Context, Result};
use colored::Colorize;
use once_cell::sync::{Lazy, OnceCell};
use regex::Regex;
use std::io;

static KEY: OnceCell<Option<Regex>> = OnceCell::new();

fn main() {
    let key = std::env::args().nth(1).unwrap_or_default();
    KEY.get_or_init(|| {
        if key.is_empty() {
            None
        } else {
            let ident = format!("(?i){key}");
            Some(
                Regex::new(&ident)
                    .with_context(|| format!("invalid key {key:?}"))
                    .unwrap(),
            )
        }
    });
    run().unwrap();
}

fn run() -> Result<()> {
    let mut buffer = String::new();
    loop {
        buffer.clear();
        let n = io::stdin().read_line(&mut buffer)?;
        if n == 0 {
            break;
        }
        line(buffer.trim_matches('\n'));
    }
    Ok(())
}

// Use `Cow` is better for performace
fn filter(s: &str) -> Option<String> {
    if let Some(key) = KEY.get().unwrap() {
        let matched = key.find(s)?;
        let colored = matched.as_str().red();
        let mut buf = s.to_string();
        buf.replace_range(matched.range(), &colored.to_string());
        Some(buf)
    } else {
        // no filter
        Some(s.to_string())
    }
}

// Use `Cow` is better for performace
fn demangle(s: &str) -> String {
    static RUST_LABLE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b_[0-9a-zA-Z_]+").unwrap());
    let mut ret = s.to_string();
    if let Some(m) = RUST_LABLE.find(s) {
        rustc_demangle::try_demangle(m.as_str())
            .map(|d| ret.replace_range(m.range(), &d.to_string()))
            .ok();
    }
    ret
}

// bloaty ./target/aarch64-apple-darwin/release/deps/libsecsdk.dylib -d symbols  --domain file -n 2000 -C full
//__RNvXs1_NtNtCsgvgX0gS3ihz_3nix5errno6constsNtB5_5ErrnoNtNtCsizNi70uxzCp_4core3fmt5Debug3fmt
fn line(s: &str) {
    if let Some(s) = filter(&demangle(s)) {
        println!("{s}")
    }
}
