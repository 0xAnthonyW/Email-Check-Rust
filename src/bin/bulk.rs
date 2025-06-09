use check_if_email_exists::{check_email, CheckEmailInput};
use std::{fs::File, io::{self, BufRead, BufReader, Write}};
use futures::future::join_all;

#[tokio::main]
async fn main() {
    // ── Prompt for path ──────────────────────────────
    print!("Enter path to email list file: ");
    io::stdout().flush().unwrap();

    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim();

    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Failed to open file: {}", path);
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut tasks = vec![];

    for line in reader.lines() {
        let email = line.unwrap();
        let email = email.trim().to_string();
        if email.is_empty() { continue; }

        let task = async move {
            let input = CheckEmailInput::new(email.clone());
            let result = check_email(&input).await;
            (email, result)
        };
        tasks.push(task);
    }

    let results = join_all(tasks).await;

    let mut safe = vec![];
    let mut risky = vec![];
    let mut unknown = vec![];
    let mut not_deliverable = vec![];
    let mut invalid_syntax = vec![];

    for (email, result) in results {
        if !result.syntax.is_valid_syntax {
            invalid_syntax.push(email);
            continue;
        }

        match &result.smtp {
            Ok(smtp) => {
                if !smtp.is_deliverable {
                    not_deliverable.push(email);
                    continue;
                }

                match &result.misc {
                    Ok(misc) => {
                        if misc.is_disposable
                            || misc.is_role_account
                            || smtp.is_catch_all
                            || smtp.has_full_inbox
                            || smtp.is_disabled {
                            risky.push(email);
                        } else {
                            safe.push(email);
                        }
                    }
                    Err(_) => unknown.push(email),
                }
            }
            Err(_) => unknown.push(email),
        }
    }

    write_list("safe.txt", &safe);
    write_list("risky.txt", &risky);
    write_list("unknown.txt", &unknown);
    write_list("not_deliverable.txt", &not_deliverable);
    write_list("invalid_syntax.txt", &invalid_syntax);

    println!("\nDone. Output written to:");
    println!("- safe.txt\n- risky.txt\n- unknown.txt\n- not_deliverable.txt\n- invalid_syntax.txt");
}

fn write_list(filename: &str, list: &[String]) {
    let mut file = File::create(filename).expect("Cannot create output file");
    for email in list {
        writeln!(file, "{}", email).unwrap();
    }
}
