# Axum github oauth

A spike on using GitHub OAuth as authentication and authorization service.

## Configuration

Environment variables for authentication:

```command
OAUTH_CLIENT_ID="<...snip...>"
OAUTH_CLIENT_SECRET="<...snip...>"
REDIRECT_URL="https://example.com/authorize"
SESSION_KEY="some-long-random-string"
```

For authorization, the following variables are needed:

```command
export REQUIRED_ORG=my-org
export REQUIRED_TEAM_SLUG=my-team
```

When using GitHub Enterprise, configure your instance with

```command
export GITHUB_API_BASE=https://github.example.com/api/v3
```

## Example

see [src/main.rs](src/main.rs).

## Running multiple instances

The whole application is stateless. The only shared part is the `SESSION_KEY`, which will be used by each instance to decrypt the cookie.

1. Start three instances of the app:

    If you want request logging, set `export RUST_LOG=tower_http=trace` before starting the server.

    ```command
    cargo run -- --bind-address localhost:3010
    cargo run -- --bind-address localhost:3011
    cargo run -- --bind-address localhost:3012
    ```

1. Start a reverse proxy to load-balance between these:

    ```command
    caddy reverse-proxy \
      --from http://localhost:3001 \
      --to http://localhost:3010 \
      --to http://localhost:3011 \
      --to http://localhost:3012
    ```

## TODO

- Allow customization of CONTACT
- redirect to original URL if it is protected and a login had to happen
