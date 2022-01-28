/*
  Main focus: Arrays, Slices, And Vecs. Mostly on Vecs, they are the most flexible. I am no leet programmer (yet).
  @dev: problem: we want to store n lines of context aroudn a match.
  we will collect lines within n that match the tags we store.
*/
use clap::{App, Arg};
use regex::Regex;

fn main() {
    // instanciate a new App from clap.
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for pattern")
        .arg(
            Arg::new("pattern")
                .help("the pattern to seach for")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // pass our cli argument into the the regex pattern.
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let quote = "Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{line}"),
            None => (), // case we throw a unit.
        }
    }

    let ctx_lines = 1;
    let needle = "oo";

    let haystack = "\
    Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek
    through millions of pages?";

    let mut tags: Vec<usize> = vec![]; // hold line numbers where matches are.
    let mut ctx: Vec<Vec<(usize, String)>> = vec![]; // contains a vec per match to hold context lines.

    for (idx, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(idx);

            let v_store = Vec::with_capacity(2 * ctx_lines + 1); // this gets inferred by the push below.
            ctx.push(v_store);
        }
    }
    // if no matches, exit early.
    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let upper_bound = tag + ctx_lines;
            let lower_bound = tag.saturating_sub(ctx_lines); // sat_sub returns 0 on integer underflow rather than crash.

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_str = String::from(line);
                let local_ctx = (i, line_as_str);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!(" {line_num}: {line}");
        }
    }
}
