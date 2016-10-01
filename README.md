# OS X Overseer

Simple, lightweight userspace process manager for OS X.

## Requirements

- Run as a server
  - Be triggered by cron job
  - Take a lock file to know if exists
- Fit in .config/overseer/
- Kick off process and restart them if they go down
- Check if process should be running with arbitrary test
