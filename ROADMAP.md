# oxideye

## 🚀 CLI as a controller for the daemon v0.1.0

- Move event listener to be a daemon, that the user interacts with via the CLI
- Use CLI as a way to `start/stop` the daemon, as well as a way to query and aggregate it’s results
    - `oxideye start --config <config> --period 10000 --log-level [info, debug] --log-path <path>` starts the daemon that listens to mouse/keyboard events
        - `config` contains **info about the data itself**, so `stats_dir` and `postfix`
        - CLI arguments contain **information about this instance of the daemon**
    - `oxideye stop` will **flush the current stats** and **stop the daemon**

## 🚀 CLI as a way to view the data v0.2.0

- Use CLI also as a way to **aggregate** and **view the data**
    - `oxideye tail` can be a way to **tail the active logs** of the daemon (if running)
    - `oxideye stats <time>` will be a way to view the raw stats
        - `<time>` should support **day**, **week**, **month**, **year** and **all time**, as well as **ranges**
        - If **applicable**, it should print out **averages**, **variance** and **total**
    - `oxideye fun-fact` will pick a random stat, and a time (day, week, month, year, all time) and come up with a fun-fact.
        - Initially, these will be **hard-coded** for certain stat/range combos.
        - **Optional:** If an LLM key is available (e.g. `--llm` or `OPENAI_API_KEY` in env), enhance or generate dynamic fun facts via an LLM.

## 🚀 light DB integration v0.3.0

Start using a DB for persistence of logs!

## 🚀 Notifications v0.4.0

Somehow push notifications from the background daemon to user-space..?

- e.g. `Your mouse went to the moon and back this week!`
- Might require different strategies per platform…

## 🚀 Synchronization  v0.5.0

Setup a remote server that people can upload/load their telemetry data to/from.

- `oxideye sync` synchronizes the remote server and your server.
    - **Question:** How do we reconcile the data?
        - Should we deduplicate by timestamp?
        - Merge logs with conflicts?
        - Should we version the data format?

## 🚀 Stable Release v1.0.0

- **Lock in** the API and main features
- Release on `brew`
- Post on **Twitter**, **reddit**, **discord**, **hacker news, linked-in**

---

## 🧹 Internals & Developer Experience

- Unit tests for:
    - Config loading
    - Error handling
