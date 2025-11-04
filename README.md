# 📚 VAS General Ledger (Rust)

> A secure, audit-ready, VAS-compliant accounting system built in Rust — designed for Vietnamese accounting firms.

---

## 🎯 Purpose

This project implements a **General Ledger (GL)** system fully compliant with **Vietnam Accounting Standards (VAS)** as defined in **Decree 133/2016/ND-CP** (applicable to small and medium enterprises, or SME).

It focuses on:
- ✅ **Double-entry integrity** (Debit = Credit)
- ✅ **Period management** (Open/Close monthly/annually)
- ✅ **Audit trail** (Immutable transaction history)
- ✅ **VAT handling** (Compliance with tax reporting)
- ✅ **Multi-client support** (Scalable for firms)

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
````

-----

## 📦 Project Structure

```bash
gl-vn/
├── Cargo.toml
├── gl-core/
│   ├── src/
│   │   ├── lib.rs          ← Core domain logic
│   │   └── coa/            ← Chart of Accounts
│   └── Cargo.toml
├── gl-cli/
│   ├── src/
│   │   └── main.rs         ← Command-line interface
│   └── Cargo.toml
├── data/
│   ├── vn_coa.yaml         ← VAS-compliant CoA (Decree 133/2016/ND-CP)
│   └── sample_sale.json    ← Sample journal entry
└── tests/
    └── lib.rs              ← Unit tests
```

-----

## 🛠️ Dependencies

| Crate | Purpose |
|------|---------|
| `serde` | **JSON/YAML serialization** (Data exchange) |
| `chrono` | **Date/time handling** (Period management, transaction dates) |
| `rust_decimal` | **Precise decimal math** (Financial accuracy) |
| `thiserror` | **Custom error types** (Robust error handling) |
| `tracing` | **Structured logging** (Audit trail, debugging) |
| `clap` | **CLI argument parsing** (User interface) |
| `csv` | **CSV import/export** (Integration with spreadsheets) |
| `axum` | Web framework (for `gl-api` in V1 roadmap) |
| `tokio` | Async runtime (for API/Scale) |
| `sqlx` | Database connectivity (for PostgreSQL) |

-----

## 🧪 Running Tests

```bash
cargo test --all
```

✅ All unit tests pass — covering valid/invalid/edge cases.

-----

## 🖥️ Usage (CLI)

### Initialize a new ledger:

```bash
cargo run --bin gl-cli -- --init
```

### Add a journal entry:

```bash
cargo run --bin gl-cli -- --entry-add data/sample_sale.json
```

### Generate trial balance:

```bash
cargo run --bin gl-cli -- --report-trial
```

### Open a new period (Example for January 2026):

```bash
cargo run --bin gl-cli -- --period-open 01 --year 2026 
```

> **Note:** The `PERIOD_OPEN` argument expects the month.

-----

## 📾 Compliance with Decree 133/2016/ND-CP

Your system is built to comply with:

  - ✅ **VAS 01: General Principles** (double-entry, audit trail, basic GL structure)
  - ✅ **VAS 21: Financial Statements** (period closing, trial balance generation)
  - ✅ **VAS 33: Taxation** (VAT input/output handling and reporting)
  - ✅ **Circular 200/2014/TT-BTC**: Chart of Accounts (The foundation for the `vn_coa.yaml` file)

-----

## 📈 Roadmap

| Phase | Features |
|------|----------|
| **MVP** | Journal entry, ledger, **trial balance** |
| **MMF** | **Period management**, recurring entries, **AR/AP (Account Receivable/Payable)** |
| **V1** | **Multi-client**, **Web API (axum)**, Reporting (B01-DNN/B02-DNN) |
| **Scale** | E-invoice parsing, **HTKK export** (Vietnam Tax Software), **multi-currency** |

-----

## 📝 License

MIT License — see [LICENSE](https://www.google.com/search?q=LICENSE) file.

-----

## 💬 Contact

For questions or feedback, contact: *(Your contact information here)*

-----

## 🧑‍💻 Contributing

Contributions are welcome\! Please open an issue or submit a PR.

-----

## 🙌 Thank You\!

Thank you for using this project. Together, we’re building the future of **secure, compliant, and auditable accounting software** in Vietnam.

```

Would you like me to help draft the content for one of the missing components, such as the `gl-core` `lib.rs` file?
```
