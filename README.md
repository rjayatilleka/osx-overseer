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
- Fit in `.config/overseer/`
- Kick off process and restart them if they go down
- Check if process should be running with arbitrary test

# Architecture

Event + State + Config -> State + Processes

## State Machine

## Types

```rust
struct Command {
  exe: String,
  args: Vec<String>
};

struct Job { 
  name: String,
  job_command: Command,
  test_command: Option<Command>,
};

struct Config(Vec<Job>);
```
