# OS X Overseer

Simple, lightweight userspace process manager for OS X.

## Requirements

- Run as a server `overseerd`
  - Persist its state somewhere
  - Bind a port to take triggers from
- Be triggered by re-running `overseer`
  - Detect new jobs
  - Detect jobs with changed configurations
  - Re-run tests for all jobs
  - Kill and kickoff processes for jobs with different state
  - Re-persist state
- Config in `$XDG_CONFIG_HOME/overseer`, default `$HOME/.config/overseer/`
- Runtime data in `$XDG_DATA_HOME/overseer`, default `$HOME/.local/share/overseer`
- Kick off process and restart them if they go down
- Check if process should be running with arbitrary test

# Architecture

Event + State + Config -> State + Processes

## State Machine

```
Start
-> CheckIfRunning

CheckIfRunning
-> DaemonRunning
-> DaemonNotRunning

DaemonRunning
-> SendTrigger

SendTrigger
-> 

Exit
ReadConfig
ReadState

```

## Types

```rust
struct Command {
  exe: String,
  args: Vec<String>,
};

struct Job { 
  name: String,
  job_command: Command,
  test_command: Option<Command>,
};

struct Config {
  jobs: Vec<Job>,
}
```
