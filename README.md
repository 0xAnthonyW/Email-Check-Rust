# Email‑Check‑Rust

A fast **CLI email verifier** built in Rust.  
It checks syntax, MX records, SMTP reachability, catch‑all status, Gravatar, disposable / role‑based indicators, and more—then prints a neat one‑page report.

---

## ✨  Highlights
| Capability | Details |
|------------|---------|
| **Async**  | Powered by `tokio` for non‑blocking DNS + SMTP look‑ups |
| **Accuracy** | Uses [`check‑if‑email‑exists`](https://crates.io/crates/check-if-email-exists) for deep validation |
| **Readable output** | Color‑free, clipboard‑friendly report (see sample below) |
| **Tiny binary** | Compiles to a single, static executable |
| **Cross‑platform** | Works on Windows, macOS, and Linux with the same flags |

---

## 🚀  Quick start

```bash
# 1. Clone
git clone https://github.com/<you>/email-check-rust.git
cd email-check-rust

# 2. Build optimized release binary
cargo build --release

# 3. Run
./target/release/email-check-rust
```

---

## 🖥️  Example session

```
Email‑Check‑Rust — press <Enter> on an empty line to quit.

Email to check: test@gmail.com

================ Verification Report ================
Address       : test@gmail.com
Overall status : RISKY — deliverable but server responded oddly
Valid syntax   : true
Has mail server: true
  • alt3.gmail-smtp-in.l.google.com.
  • alt1.gmail-smtp-in.l.google.com.
  • alt4.gmail-smtp-in.l.google.com.
  • alt2.gmail-smtp-in.l.google.com.
  • gmail-smtp-in.l.google.com.
Server reachable: true
Deliverable     : false
Mailbox disabled: false
Inbox full      : false
Catch‑all domain : false
Disposable email : false
Role‑based addr  : true  ← shared mailbox like support@ or admin@
Gravatar avatar : https://www.gravatar.com/avatar/1aedb8d9dc4751e229a335e371db8058?d=identicon
====================================================
```

---

## 🧩  Crate features

| Dependency | Purpose |
|------------|---------|
| `check-if-email-exists` | High‑level verification engine |
| `tokio` with `rt‑multi‑thread`, `macros`, `net` | Async runtime |
| `md5` | Hash helper for Gravatar URLs |

---

## ⚙️  Configuration

| Option | Environment variable | Default | Description |
|--------|----------------------|---------|-------------|
| **Timeout** | `ECE_TIMEOUT_MS` | `10000` | Max wait per SMTP probe |
| **Parallel checks** | `ECE_CONCURRENCY` | Number of CPU cores | Tokio worker threads |

---

## 📦  Packaging tips

```bash
# Build a musl‑linked binary (static, ~3 MB)
cargo build --release --target x86_64-unknown-linux-musl
strip target/x86_64-unknown-linux-musl/release/email-check-rust
```

Distribute the resulting file; it has **zero runtime dependencies**.

---

## 🔒  Security notes
The program **never stores** or forwards email addresses. Every check happens locally over DNS + SMTP, and results are printed directly to the console.

---

## 📝  License
Code is distributed under the MIT License.  
See `LICENSE` for full text.

---
