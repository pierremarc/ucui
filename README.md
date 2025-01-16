# ucui

A minimal UCI engine frontend experiment.

It's intended to be used as a least disruptive interface when playing over the board with a chess engine.

It's a work in progress. If you want to give it a try, you'll have to compile it from source. For this you'll need to [install `rust`](https://www.rust-lang.org/tools/install), clone or download this repository, then `cargo run --release -- --help` (release builds are very long to compile, but it happens only once) from the directory where the code ended up.

Note that it embeds the [blunders](https://github.com/paulolemus/blunders/) chess engine that is enough to test the interface. To play with a more advanced engine, you need to instruct `ucui` to do so, e.g.:

```
$ which stockfish
/usr/games/stockfish
$ cargo run --release -- --engine /usr/games/stockfish --white-time 1200 --black-time 300
```

![](screenshot.png)

```
Usage: ucui [OPTIONS]

Options:
  -e, --engine <ENGINE>
          Path to a UCI engine
  -w, --white-time <TIME>
          White time in seconds [default: 600]
  -b, --black-time <TIME>
          Black time in seconds [default: 600]
  -c, --engine-color <COLOR>
          set engine color [default: black] [possible values: white, black]
  -f, --fen <FEN>
          Optional starting position in FEN format
      --engine-args <ARGS>
          Optional arguments to pass to the engine (separated by ";")
      --log-level <LOG_LEVEL>
          set log level [default: info] [possible values: off, error, warn, info, debug, trace]
      --uci-debug-log <UCI_DEBUG_LOG>

      --uci-threads <UCI_THREADS>

      --uci-hash <UCI_HASH>

      --uci-skill-level <UCI_SKILL_LEVEL>

      --uci-move-overhead <UCI_MOVE_OVERHEAD>

      --uci-slow-mover <UCI_SLOW_MOVER>

      --uci-nodestime <UCI_NODESTIME>

      --uci-uci-limit-strength <UCI_UCI_LIMIT_STRENGTH>

      --uci-uci-elo <UCI_UCI_ELO>

      --uci-syzygy-path <UCI_SYZYGY_PATH>

      --uci-syzygy-probe-depth <UCI_SYZYGY_PROBE_DEPTH>

      --uci-syzygy50-move-rule <UCI_SYZYGY50_MOVE_RULE>

      --uci-syzygy-probe-limit <UCI_SYZYGY_PROBE_LIMIT>

      --uci-use-nnue <UCI_USE_NNUE>

      --uci-eval-file <UCI_EVAL_FILE>

  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```
