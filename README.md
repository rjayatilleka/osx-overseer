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

## Daemon State Machine

- Start
  - -> InitHomes
- InitHomes
  - Success -> OpenSocket(Homes)
  - Failure -> Exit(Err(InitHomesError))
- OpenSocket(Homes)
  - Success -> ReceiveRequest(Socket)
  - Failure -> Exit(Err(SocketOpenError))
- ReceiveRequest(Socket)
  - Success -> ProcessRequest(Socket, Request)
  - Failure -> Exit(Err(SocketReadError))
- ProcessRequest(Socket, Request)
  - "DIE" -> Exit(Ok(()))
  - other -> SendResponse(Socket)
- SendResponse(Socket)
  - Success -> ReceiveRequest(Socket)
  - Failure -> Exit(Err(SocketWriteError))
- Exit(Result<(), Error>)
  - -> Done
- Done

## Client State Machine

- Start
  - -> InitHomes
- InitHomes
  - Success -> CheckSocketExists(Homes)
  - Failure -> Exit(Err(InitHomesError))
- CheckSocketExists(Homes)
  - Success -> OpenSocket(Homes)
  - Failure -> LaunchDaemon
- OpenSocket(Homes)
  - Success -> SendRequest(Socket)
  - Failure -> DeleteSocketFile(Homes)
- SendRequest(Socket)
  - Success -> ReadResponse(Socket)
  - Failure -> CloseSocket(Socket, Exit(Err(SocketSendError)))
- CloseSocket(Socket, State)
  - -> param state
- DeleteSocketFile(Homes)
  - -> LaunchDaemon(Homes)
- LaunchDaemon(Homes)
  - Success -> OpenSocket(Homes)
  - Failure -> Exit(Err(DaemonLaunchError))
- ReadResponse(Socket)
  - Success -> CloseSocket(Socket, PrintResponse(Response))
  - Failure -> CloseSocket(Socket, Exit(Err(SocketReadError)))
- PrintResponse(Response)
  - -> Exit(Ok(()))
- Exit(Result<(), Error>)
  - -> Done
- Done

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
