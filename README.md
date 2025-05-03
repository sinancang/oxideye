# oxideye 👁️‍🗨️

**oxideye** records and logs **computer peripheral usage stats** (think mouse distance, key/button press, etc.) so you can brag to your friends about how many miles your mouse has travelled

---
**Currently** the program logs:
- Mouse distance (in pixels)
- Wheel spins
- Button presses
- Key presses

---
**Down the line** I intend to add:
- CLI tool to easily access the stats, as well as fun-facts (e.g. "if you had a cent for each time you pressed a key, you'd have $X!")
- Notifications on milestones (when stats exceed certain thresholds)
- Stat sharing across multiple devices

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
oxideye will, by default write logs to `oxideye/telem.json` every `100000` milliseconds. Both of these are configurable via modifying `config/default.toml`
