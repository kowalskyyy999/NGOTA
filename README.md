# NGOTA
**Ngocok Data** - In english its mean scrambling data. NGOTA is a tool for masking data with contains sensitive data such as name, age, address and so on. This tool not only for masking data in single csv file, but support multiple csv depends with the same field/column. So this is supporting data privacy while use for data analysis.

# How to Use
1. Ensure your system has installed Rust. If not install yet, please [install](https://www.rust-lang.org/tools/install) first
2. Clone this repo
```bash
git clone https://github.com/kowalskyyy999/NGOTA.git
```
3. Change directory
```bash
cd NGOTA
```
4. Compile the script and dependencies
```bash
cargo build
```
5. Masking data with Single File
```bash
./target/debug/ngota --files data/file1.csv --config-file config/single-config.yaml
```
6. Masking data with Multiple Files
```bash
./target/debug/ngota --files data --config-file config/multi-config.yaml --multi
```