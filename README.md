```markdown
# 📚 VAS General Ledger (Rust)

> A secure, audit-ready, VAS-compliant accounting system built in Rust — designed for Vietnamese accounting firms.

---

## 🎯 Purpose

This project implements a **General Ledger (GL)** system fully compliant with **Vietnam Accounting Standards (VAS)** as defined in **Decree 133/2016/ND-CP**.

It focuses on:
- ✅ Double-entry integrity
- ✅ Period management
- ✅ Audit trail
- ✅ VAT handling
- ✅ Multi-client support

Built with **Rust** for performance, memory safety, and concurrency — ideal for financial software.

---

## 🧩 Architecture

```mermaid
graph TD
    A[CLI / Web UI] --> B[gl-api (axum)]
    B --> C[gl-core (Domain Logic)]
    C --> D[PostgreSQL (ACID)]
    C --> E[Tracing / Logging]
    C --> F[CSV / JSON I/O]
    C --> G[Report Generator (B01-DNN, B02-DNN)]
    D --> H[Audit Trail (Immutable Log)]
    H --> I[Compliance with Decree 133/2016/ND-CP]
```

---

## 📦 Project Structure

```bash
gl-vn/
├── Cargo.toml
├── gl-core/
│   ├── src/
│   │   ├── lib.rs          ← Core domain logic
│   │   └── coa/            ← Chart of Accounts
│   └── Cargo.toml
├── gl-cli/
│   ├── src/
│   │   └── main.rs         ← Command-line interface
│   └── Cargo.toml
├── data/
│   ├── vn_coa.yaml         ← VAS-compliant CoA
│   └── sample_sale.json    ← Sample journal entry
└── tests/
    └── lib.rs              ← Unit tests
```

---

## 🛠️ Dependencies

| Crate | Purpose |
|------|---------|
| `serde` | JSON/YAML serialization |
| `chrono` | Date/time handling |
| `rust_decimal` | Precise decimal math |
| `thiserror` | Custom error types |
| `tracing` | Structured logging |
| `clap` | CLI argument parsing |
| `csv` | CSV import/export |

---

## 🧪 Running Tests

```bash
cargo test --all
```

✅ All unit tests pass — covering valid/invalid/edge cases.

---

## 🖥️ Usage (CLI)

### Initialize a new ledger:

```bash
cargo run --bin gl-cli -- init
```

### Add a journal entry:

```bash
cargo run --bin gl-cli -- entry-add data/sample_sale.json
```

### Generate trial balance:

```bash
cargo run --bin gl-cli -- report-trial
```

---

## 📾 Compliance with Decree 133/2016/ND-CP

Your system is built to comply with:

- ✅ VAS 01: General Principles (double-entry, audit trail)
- ✅ VAS 21: Financial Statements (period closing, trial balance)
- ✅ VAS 33: Taxation (VAT input/output)
- ✅ Circular 200/2014/TT-BTC: Chart of Accounts

---

## 📈 Roadmap

| Phase | Features |
|------|----------|
| **MVP** | Journal entry, ledger, trial balance |
| **MMF** | Period management, recurring entries, AR/AP |
| **V1** | Multi-client, web API, reporting (B01-DNN/B02-DNN) |
| **Scale** | E-invoice parsing, HTKK export, multi-currency |

---

## 📝 License

MIT License — see [LICENSE](LICENSE) file.

---

## 💬 Contact

For questions or feedback, contact:  


---

## 🧑‍💻 Contributing

Contributions are welcome! Please open an issue or submit a PR.

---

## 🙌 Thank You!

Thank you for using this project. Together, we’re building the future of **secure, compliant, and auditable accounting software** in Vietnam.

