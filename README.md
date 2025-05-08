# oxideyes 👁️‍🗨️

**oxideye** records and logs **computer peripheral usage stats** (think mouse distance, key/button press, etc.) so you can brag to your friends about how many miles your mouse has travelled


## Usage

### Clone the repository:
```bash
git clone https://github.com/sinancang/oxideye.git
cd oxideye
```

### Run the project:
```bash
cargo run
```

Alternatively, run the program in debug mode to see every time the process is notified of a usage event, every time an update goes through, etc.
```bash
cargo run -- --log-level debug
```

### Let it run!
oxideye will, by default flush stats to `./data/YYYYMMDD_telem.json` every `10000` milliseconds. The stats directory (`data`), as well postfix (`telem`) can be configured via `config/default.toml`.
