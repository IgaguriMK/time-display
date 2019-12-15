mod display;
mod time;

use std::fs::create_dir_all;
use std::io::{stdin, stdout, BufRead, Write};
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches, SubCommand};
use tiny_fail::{Fail, FailExt};

use display::Display;
use time::Time;

fn main() {
    if let Err(e) = w_main() {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}

fn w_main() -> Result<(), Fail> {
    let matches = App::new("time-display")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .default_value("output.png")
                .help("Output to specified PNG file."),
        )
        .arg(
            Arg::with_name("time")
                .takes_value(true)
                .help("Specify time to generate. (ex: 1:23.45)"),
        )
        .subcommand(
            SubCommand::with_name("bulk")
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .default_value("frames")
                        .help("Output directory."),
                )
                .arg(
                    Arg::with_name("framerate")
                        .short("r")
                        .long("framerate")
                        .default_value("60")
                        .help("Output framerate"),
                )
                .arg(
                    Arg::with_name("len")
                        .takes_value(true)
                        .help("Specify length to generate. (ex: 1:23.45)"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("bulk") {
        return bulk(matches).context("while executing subcommand 'bulk'");
    }

    single(&matches).context("while executing main command")
}

fn single(matches: &ArgMatches) -> Result<(), Fail> {
    let t = if let Some(ts) = matches.value_of("time") {
        Time::parse(ts).context("failed parse time")?
    } else {
        read_time().context("while reading time from input")?
    };

    let out = matches.value_of("output").unwrap();

    let display = Display::builtin();
    let img = display
        .print(&t.to_string())
        .context("while generating image")?;

    img.save(out).context("while saving image")?;

    Ok(())
}

fn bulk(matches: &ArgMatches) -> Result<(), Fail> {
    let time_limit = if let Some(ts) = matches.value_of("len") {
        Time::parse(ts).context("failed parse time")?
    } else {
        read_time().context("while reading time from input")?
    };

    let rate = matches
        .value_of("framerate")
        .unwrap()
        .parse::<f64>()
        .context("failed parse framerate")?;

    let out = matches.value_of("output").unwrap();
    let out_dir = PathBuf::from(out);
    if !out_dir.exists() {
        create_dir_all(&out_dir).context("failed create output directory")?;
    }

    let display = Display::builtin();

    let frame_len = 1.0 / rate;
    let frames = time_limit.secs_f64() / frame_len;
    let digits = 1 + (frames.log10() as usize);

    let mut t = Time::zero();
    let mut idx = 0usize;
    while t < time_limit {
        let img = display
            .print(&t.to_string())
            .context(format!("while generating image for {}", t))?;

        let file_name = format!("{v:0digits$}.png", digits = digits, v = idx);
        let file_path = out_dir.join(&file_name);
        img.save(&file_path)
            .context(format!("while saving image {}", file_name))?;

        t += frame_len;
        idx += 1;
    }

    Ok(())
}

fn read_time() -> Result<Time, Fail> {
    let out = stdout();
    let mut out_lock = out.lock();

    let stdin = stdin();
    let mut in_lock = stdin.lock();
    let mut buf = String::new();

    loop {
        write!(&mut out_lock, "Enter time (ex: 1:23.45): ")?;
        out_lock.flush()?;
        buf.truncate(0);
        in_lock.read_line(&mut buf)?;

        match Time::parse(buf.trim()) {
            Ok(t) => return Ok(t),
            Err(e) => {
                eprintln!("Invalid input: {}", e);
            }
        }
    }
}
