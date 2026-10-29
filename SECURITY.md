# Security Policy

Zacess Games are small programs that take over a terminal, read a few settings, run Lua levels
and play synthesised sound. A flaw that lets a level or a setting do more than that is taken
seriously.

## Scope

In scope:

- A Lua level escaping its sandbox: reading or writing files, opening a network connection, running
  another program or using more memory or time than its limits allow
- A game reading or writing anything beyond what its guide documents, such as its settings and
  saved scores
- A crafted level, save file, settings file or environment variable that makes a game misbehave
- The terminal left in raw mode or on the alternate screen after a quit, a panic or `Ctrl+C`
- The release workflow publishing something other than what the tagged commit builds or exposing
  a token

## Out of scope

- How a terminal itself handles colours, the alternate screen or raw mode (report those to its
  makers)
- Vulnerabilities in Zacess Scenes (report those to
  [zacesshq/scenes](https://github.com/zacesshq/scenes/security/advisories/new))
- Vulnerabilities in Lua, the Rust crates, GitHub Actions or the Rust toolchain with no
  demonstrated impact on the games (report those upstream)

## Supported versions

Only the latest release and the latest commit on `main` receive security fixes.

## Reporting a vulnerability

> [!IMPORTANT]
> Report privately, never in a public issue or discussion. Use
> [GitHub private vulnerability reporting](https://github.com/zacesshq/games/security/advisories/new)
> or email [contact@zacess.com](mailto:contact@zacess.com) with details and reproduction steps.
> Expect an acknowledgement within a few days.
