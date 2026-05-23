# findfin

A standalone Jellyfin auto-discovery responder for setups where Jellyfin runs in Docker without host networking.

## Configuration

All configuration is done via environment variables.

| Env Var | Required | Description |
|---|---|---|
| `FINDFIN_SERVER_URL` | Yes | URL of the Jellyfin server, used to query server info and as the default announced address, e.g. `http://jellyfin.local:8096` |
| `FINDFIN_ANNOUNCE_URL` | No | URL to announce to clients; overrides `FINDFIN_SERVER_URL` in discovery responses, e.g. `https://jellyfin.example.com` |
| `FINDFIN_BIND_ADDR` | No | IP address to listen on (defaults to `0.0.0.0`) |

Server ID and name are automatically fetched from Jellyfin at startup.

## How to run

Build and use the included systemd service.
