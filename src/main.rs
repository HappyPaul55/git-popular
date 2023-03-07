use chrono::{NaiveDate, Datelike};
use clap::Parser;
use rand::Rng;
use std::{process, os::windows::process::CommandExt};

/// A simple way to make a repo look populated.
#[derive(Parser, Debug)]
#[command(author="Paul Hutchinson", version="v1.0.0", about, long_about = None)]
struct Args {
    /// When to start creating commits.
    #[arg(short, long)]
    start: NaiveDate,

    /// When to stop creating commits.
    #[arg(short, long)]
    end: NaiveDate,

    /// Extra arguments to pass to git.
    #[arg(short, long)]
    git: Option<String>,

    /// Working directory, must have existing git repo setup.
    #[arg(short, long)]
    dir: String,

    /// Maxiumn number of commits on a Monday?
    #[arg(short = '1', long, default_value_t = 0)]
    monday: u16,

    /// Maxiumn number of commits on a Tuesday?
    #[arg(short = '2', long, default_value_t = 0)]
    tuesday: u16,

    /// Maxiumn number of commits on a Wednesday?
    #[arg(short = '3', long, default_value_t = 0)]
    wednesday: u16,

    /// Maxiumn number of commits on a Thursday?
    #[arg(short = '4', long, default_value_t = 0)]
    thursday: u16,

    /// Maxiumn number of commits on a Friday?
    #[arg(short = '5', long, default_value_t = 0)]
    friday: u16,

    /// Maxiumn number of commits on a Saturday?
    #[arg(short = '6', long, default_value_t = 0)]
    saturday: u16,

    /// Maxiumn number of commits on a Sunday?
    #[arg(short = '7', long, default_value_t = 0)]
    sunday: u16,
}

fn main() {
    let args = Args::parse();

    if args.start > args.end {
        println!(
            "--start \"{:?}\" cannot be after --end \"{:?}\"",
            args.start, args.end
        );
        process::exit(1);
    }

    let mut current_date = args.start;
    let mut rng = rand::thread_rng();
    let day_of_the_week_factors: [u16; 7] = [
        args.monday,
        args.tuesday,
        args.wednesday,
        args.thursday,
        args.friday,
        args.saturday,
        args.sunday,
    ];

    let git_args: String = args.git.unwrap_or_default();
    let dir: String = args.dir;
    while current_date <= args.end {
        let days_from_monday: usize = current_date.weekday().number_from_monday() as usize;
        let factor: u16 = day_of_the_week_factors[days_from_monday - 1];

        let commits_to_make: u16 = rng.gen_range(0..=factor);
        println!("Making {:?} commits on {:?}", commits_to_make, current_date);
        let mut commits_made: u16 = 0;
        while commits_made < commits_to_make {
            println!(
                "{} {}:{}:{}",
                current_date.to_string(),
                format!("{:0>2}", (commits_made / 60 / 60)),
                format!("{:0>2}", (commits_made / 60) % 60),
                format!("{:0>2}", commits_made % 60)
            );
            
            let output = process::Command::new("git")
                .current_dir(&dir)
                .arg("commit")
                .arg("-m")
                .arg(format!("Comment #{:?} on {:?}", commits_made + 1, current_date))
                .arg("--allow-empty")
                .arg("--date")
                .arg(format!(
                    "{} {}:{}:{}",
                    current_date.to_string(),
                    format!("{:0>2}", (commits_to_make / 60 / 60)),
                    format!("{:0>2}", (commits_to_make / 60) % 60),
                    format!("{:0>2}", commits_to_make % 60)
                ))
                .raw_arg(&git_args)
                .output()
                .expect("Failed to execute git command");

            if !output.status.success() {
                println!("Error: {:#?}", String::from_utf8_lossy(&output.stderr));
            }

            commits_made+= 1;
        }

        current_date = current_date.succ_opt().unwrap();
    }
}