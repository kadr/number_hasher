use sha256::digest;
use std::env;
use tokio;

const RANGE_LIMIT: u64 = 1000_000_000;
const HELP_HINT: &str = r#"
Help:
    -N - zero count at the end of sha256 hash
    -F - show result count
Example:
    cargo run -- -N 4 -F 10
"#;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] != "-N" {
        panic!(
            "{}",
            format!("Argument -N requires an argument. {}", HELP_HINT)
        );
    }
    if args[3] != "-F" {
        panic!(
            "{}",
            format!("Argument -F requires an argument. {}", HELP_HINT)
        );
    }
    let mut end_zero_count: u8 = 0;
    let mut show_count: u8 = 0;
    if let Some(f) = args.get(2) {
        if let Ok(f) = f.parse::<u8>() {
            show_count = f;
        } else {
            panic!("{}", format!("Argument -N must be a number. {}", HELP_HINT));
        }
    } else {
        panic!("{}", format!("Argument -N must be present. {}", HELP_HINT));
    }
    if let Some(n) = args.get(4) {
        if let Ok(n) = n.parse::<u8>() {
            end_zero_count = n;
        } else {
            panic!("{}", format!("Argument -F must be a number. {}", HELP_HINT));
        }
    } else {
        panic!("{}", format!("Argument -F must be present. {}", HELP_HINT));
    }

    for res in calc_hash(end_zero_count, show_count).await {
        println!("{}", res);
    }
}

async fn calc_hash(end_zero_count: u8, show_count: u8) -> Vec<String> {
    let range_count = (RANGE_LIMIT as f64 / show_count as f64).ceil() as u64;
    let mut tasks = Vec::with_capacity(show_count as usize);
    for p in 0..show_count {
        let start = p as u64 * range_count;
        let end = p as u64 * range_count + range_count;
        tasks.push(tokio::spawn(async move {
            let mut result = String::new();
            for n in start..end {
                let hash = digest(n.to_string());
                let mut characters = hash.chars().map(|c| c.to_string()).collect::<Vec<String>>();
                characters.reverse();
                if characters[..end_zero_count as usize].join("") == "0".repeat(end_zero_count as usize)
                {
                    result = format!("{}, \"{}\"", n, hash);
                    break;
                }
            }
            result
        }))
    }

    let mut items: Vec<String> = Vec::with_capacity(show_count as usize);
    for task in tasks {
        items.push(task.await.unwrap());
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calc_hash() {
        let result = calc_hash(4, 10).await;
        assert_eq!(result.len(), 10);
    }
}
