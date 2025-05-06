use core::fmt;
use std::ops::Range;

use crate::lexer::span::Span;

pub fn ctx_line(base: &str, offset: usize) -> &str {
    &base[extract_line_info(base, offset).2]
}

// (line, col, line_range)
fn extract_line_info(base: &str, offset: usize) -> (usize, usize, Range<usize>) {
    let mut line = 0;

    let start_line = base[..offset]
        .bytes()
        .enumerate()
        .fold(0, |prev, (idx, c)| {
            (c == b'\n')
                .then_some(idx + 1)
                .inspect(|_| line += 1)
                .unwrap_or(prev)
        });

    let end_line = base[start_line..]
        .bytes()
        .position(|c| (c == b'\n'))
        .map(|offset| offset + start_line)
        .unwrap_or(base.len());

    (line, offset - start_line, start_line..end_line)
}

fn raise_format(
    line: usize,
    ctx: impl fmt::Display,
    msg: impl fmt::Display,
    cursor_offset: usize,
    cursor_len: usize,
) -> ! {
    let line_digits = line.checked_ilog10().unwrap_or(0) as usize + 1;

    println!("\x1b[31merror: \x1b[1m{msg}\x1b[0m");
    println!("\x1b[36m {} \x1b[34m| \x1b[0m{ctx}", line);
    println!(
        "\x1b[36m {0:<line_digits$} \x1b[34m| \x1b[31m{0:<cursor_offset$}{0:^<cursor_len$}",
        ""
    );

    std::process::exit(1)
}

pub fn raise_at(base: &str, offset: usize, msg: impl fmt::Display) -> ! {
    let (line, col, range) = extract_line_info(base, offset);
    raise_format(line, &base[range], msg, col, 1)
}

pub fn raise_range(base: &str, span: Span, msg: impl fmt::Display) -> ! {
    let (line, col, range) = extract_line_info(base, span.from);
    raise_format(
        line,
        &base[range],
        msg,
        col,
        span.to.saturating_sub(span.from).max(1),
    )
}
