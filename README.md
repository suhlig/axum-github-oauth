# Axum github oauth

A spike on using GitHub OAuth as authentication and authorization service.

## Configuration

Environment variables:

```
OAUTH_CLIENT_ID="<...snip...>"
OAUTH_CLIENT_SECRET="<...snip...>"
REDIRECT_URL="https://example.com/authorize"
SESSION_KEY="some-long-random-string"
```

## Example

see [src/main.rs](src/main.rs).
