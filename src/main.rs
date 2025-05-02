use check_if_email_exists::{check_email, CheckEmailInput, Reachable};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!("Email‑Check‑Rust — press <Enter> on an empty line to quit.\n");

    loop {
        // ── prompt ───────────────────────────────────────────────────────────
        print!("Email to check: ");
        io::stdout().flush().unwrap();

        let mut email = String::new();
        io::stdin().read_line(&mut email).unwrap();
        let email = email.trim().to_string();

        if email.is_empty() {
            println!("\nDone.");
            break;
        }

        // ── verify ───────────────────────────────────────────────────────────
        let result = check_email(&CheckEmailInput::new(email.clone())).await;

        // ── report ───────────────────────────────────────────────────────────
        println!("\n================ Verification Report ================");
        println!("Address       : {email}");

        let reachability = match result.is_reachable {
            Reachable::Safe => "SAFE — mail should arrive",
            Reachable::Risky => "RISKY — deliverable but server responded oddly",
            Reachable::Invalid => "INVALID — server rejects mail (hard bounce)",
            Reachable::Unknown => "UNKNOWN",
        };
        println!("Overall status : {reachability}");

        println!("Valid syntax   : {}", result.syntax.is_valid_syntax);

        // ── MX records ───────────────────────────────────────────────────────
        match &result.mx {
            Ok(mx) => match &mx.lookup {
                Ok(lookup) => {
                    let has_mx = lookup.iter().next().is_some();
                    println!("Has mail server: {has_mx}");
                    if has_mx {
                        for rec in lookup.iter() {
                            println!("  • {}", rec.exchange());
                        }
                    }
                }
                Err(e) => println!("Mail server lookup error: {e:?}"),
            },
            Err(e) => println!("Mail server check error : {e:?}"),
        }

        // ── SMTP details ──────────────────────────────────────────────────────────
        match &result.smtp {
            Ok(smtp) => {
                println!("Server reachable: {}", smtp.can_connect_smtp);
                println!("Deliverable     : {}", smtp.is_deliverable);
                println!("Mailbox disabled: {}", smtp.is_disabled);
                println!("Inbox full      : {}", smtp.has_full_inbox);
                println!(
                    "Catch‑all domain : {}{}",
                    smtp.is_catch_all,
                    if smtp.is_catch_all {
                        "  ← accepts mail sent to ANY address on this domain"
                    } else {
                        ""
                    }
                );
            }
            Err(e) => println!("SMTP check error : {e:?}"),
        }

        // ── Misc details ─────────────────────────────────────────────────────────
        match &result.misc {
            Ok(misc) => {
                println!(
                    "Disposable email : {}{}",
                    misc.is_disposable,
                    if misc.is_disposable {
                        "  ← temporary/throw‑away inbox provider"
                    } else {
                        ""
                    }
                );
                println!(
                    "Role‑based addr  : {}{}",
                    misc.is_role_account,
                    if misc.is_role_account {
                        "  ← shared mailbox like support@ or admin@"
                    } else {
                        ""
                    }
                );
                // email already in scope
                let email_hash = format!("{:x}", md5::compute(email.to_lowercase().trim()));
                let gravatar = format!("https://www.gravatar.com/avatar/{email_hash}?d=identicon");

                println!("Gravatar avatar : {gravatar}");
            }
            Err(e) => println!("Misc check error : {e:?}"),
        }

        println!("====================================================\n");
    }
}
