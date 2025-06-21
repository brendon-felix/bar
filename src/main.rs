use clap::Parser;

const FULL_BLOCK: &str = "█";
const SEVEN_EIGHTHS_BLOCK: &str = "▉";
const SIX_EIGHTHS_BLOCK: &str = "▊";
const FIVE_EIGHTHS_BLOCK: &str = "▋";
const FOUR_EIGHTHS_BLOCK: &str = "▌";
const THREE_EIGHTHS_BLOCK: &str = "▍";
const TWO_EIGHTHS_BLOCK: &str = "▎";
const ONE_EIGHTHS_BLOCK: &str = "▏";
const EMPTY_BLOCK: &str = " ";

// const FULL_BLOCK: &str = "█";
// const SEVEN_EIGHTHS_BLOCK: &str = "█";
// const SIX_EIGHTHS_BLOCK: &str = "▓";
// const FIVE_EIGHTHS_BLOCK: &str = "▓";
// const FOUR_EIGHTHS_BLOCK: &str = "▒";
// const THREE_EIGHTHS_BLOCK: &str = "▒";
// const TWO_EIGHTHS_BLOCK: &str = "░";
// const ONE_EIGHTHS_BLOCK: &str = "░";
// const EMPTY_BLOCK: &str = "░";

#[derive(Parser, Debug)]
struct Args {
    proportion: f32,

    #[arg(short, long)]
    length: Option<usize>,
}

fn create_bar(length: usize, proportion: f32) -> Result<String, &'static str> {
    if length <= 0 {
        return Err("Length must be greater than 0");
    }
    if proportion < 0.0 || proportion > 1.0 {
        return Err("Proportion must be between 0.0 and 1.0");
    }
    let abs_proportion = length as f32 * proportion;
    let length_full = abs_proportion.floor();
    let filled_full = FULL_BLOCK.repeat(length_full as usize);
    let partial = (abs_proportion - length_full) * 8.0;
    let filled = if partial > 0.0 {
        let last = match partial.round() {
            0.0 => EMPTY_BLOCK,
            1.0 => ONE_EIGHTHS_BLOCK,
            2.0 => TWO_EIGHTHS_BLOCK,
            3.0 => THREE_EIGHTHS_BLOCK,
            4.0 => FOUR_EIGHTHS_BLOCK,
            5.0 => FIVE_EIGHTHS_BLOCK,
            6.0 => SIX_EIGHTHS_BLOCK,
            7.0 => SEVEN_EIGHTHS_BLOCK,
            _ => FULL_BLOCK,
        };
        filled_full + last
    } else {
        filled_full
    };
    let remaining_length = length.saturating_sub(filled.chars().count());
    let remaining = EMPTY_BLOCK.repeat(remaining_length);
    Ok(format!("{}{}", filled, remaining))
}

fn main() {
    let args = Args::parse();
    match create_bar(args.length.unwrap_or(12), args.proportion) {
        Ok(bar) => println!("{}", bar),
        Err(e) => {eprintln!("{}", e); std::process::exit(1);},
    }
}
