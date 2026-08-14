# ChatGPT access to local and hosted Aicadia over stateless MCP

Date: 2026-08-12

Status: research corrected after User direction; it does not change the current
game contract or authorize implementation

## Question

How can one person play the local Aicadia World from a conversation in the ChatGPT
app now, while taking a direct path toward a hosted Aicadia that any person can
reach from any location and from an Agent started in any folder?

## Confirmed design constraint

Aicadia will use only the current stateless MCP `2026-07-28` profile. Aicadia will
not add or translate to the older `initialize` / transport-session lifecycle merely
to support a host. The relevant Codex feature is:

```text
--enable mcp_2026_07_28
```

and its persistent configuration equivalent is:

```toml
[features]
mcp_2026_07_28 = true
```

The current `tools/aicadia-agent` already passes the command-line flag. This
research therefore rejects the previously considered legacy compatibility facade.
A prospective host must prove that it can use Aicadia's stateless discovery and
request profile; otherwise that host is not currently an Aicadia player surface.

## Executive conclusion

There are two useful but distinct ChatGPT-app routes:

1. **Available locally now:** use a Codex conversation in the ChatGPT desktop app.
   The desktop Codex host shares `~/.codex/config.toml` with Codex CLI, can connect
   directly to a Streamable HTTP server on `127.0.0.1`, and can enable
   `mcp_2026_07_28`. This is the shortest path that preserves Aicadia's chosen MCP
   revision.
2. **Chat/Work and future public distribution:** use an MCP-backed ChatGPT plugin
   only after the hosted plugin client proves support for stateless MCP
   `2026-07-28`. Local reachability would then use Secure MCP Tunnel; production
   would use one stable public HTTPS endpoint plus OAuth. OpenAI's public ChatGPT
   plugin documentation currently documents remote Streamable HTTP, server
   instructions and the older initialization terminology, but it does not document
   a user-settable `mcp_2026_07_28` flag for the hosted Chat/Work client. Do not
   build around an assumption that this private/local Codex flag also controls the
   hosted ChatGPT connector.

A skill is not the connection. The MCP server owns live data, tools, authorization,
structured results and the provider-neutral Agent contract. A skill may later be a
thin activation/workflow layer in a distributable plugin, but correct play must not
depend on it.

## Current Aicadia facts

- `./tools/aicadia-local` starts one persistent local World on loopback, provisions
  one stable development User and stores the selected database plus User UUID in
  `.aicadia-local/profile.json`.
- HTTP and MCP expose the same thirteen player capabilities. `create_user` remains
  provisioning-only.
- Contextual operations currently receive `Aicadia-User-Id`; the current contract
  explicitly says this UUID is development context, not login, authentication or
  authorization.
- MCP supports only stateless `2026-07-28`. Every request carries protocol, client
  and capability metadata; `server/discover.instructions` publishes the global
  play contract; transport sessions and `initialize` are absent.
- `./tools/aicadia-agent` enables `mcp_2026_07_28`, requires Aicadia MCP, injects
  the player contract, uses an empty workspace and isolated configuration, and
  removes that environment on exit.
- The server binds only to loopback and rejects foreign browser origins.

These facts come from [Local play](../game/local-play.md),
[Agent interface](../game/agent-interface.md),
[the local player adapter](../../tools/aicadia-agent),
[the MCP server implementation](../../src/server.rs) and
[Current MCP Agent guidance](current-mcp-agent-guidance.md).

## What the OpenAI surfaces establish

### Codex inside the ChatGPT desktop app

OpenAI documents that the ChatGPT desktop app's Codex host, Codex CLI and the IDE
extension share MCP configuration. The desktop app can add a local or remote
Streamable HTTP server, and user-level configuration lives in
`~/.codex/config.toml`; it is therefore independent of the current repository
folder. [OpenAI's Codex MCP documentation](https://learn.chatgpt.com/docs/extend/mcp)

The installed Codex CLI exposes `mcp_2026_07_28` as an under-development feature,
and the general `--enable <FEATURE>` switch enables it for a process. Aicadia's
launcher already uses that exact route. This is local product evidence, not a claim
that every OpenAI-hosted connector has the same flag.

### ChatGPT Chat/Work developer mode

ChatGPT developer mode creates a draft app for a **remote** MCP server. OpenAI
documents Streamable HTTP and SSE, OAuth/no-auth/mixed auth, tool refresh, server
instructions, arbitrary read/write tools and confirmation behavior. Eligibility is
documented for Plus, Pro, Business, Enterprise and Education accounts on the web.
[ChatGPT Developer mode](https://developers.openai.com/api/docs/guides/developer-mode)

The public documentation does not expose a per-app or per-conversation
`mcp_2026_07_28` switch. Examples still describe instructions returned during
initialization. Therefore a ChatGPT Chat/Work path is a compatibility question to
test against the real hosted client, not a reason to add legacy behavior to
Aicadia.

### Public plugins

Published plugins are documented across ChatGPT web, desktop and mobile, and Codex
on supported surfaces. They package skills, MCP-backed connectors, or both. Public
submission requires a stable public HTTPS MCP endpoint and reviewed metadata; a
Secure MCP Tunnel is for private development or private deployments, not public
directory distribution. [Plugins](https://learn.chatgpt.com/docs/plugins)
[Connect and test a plugin](https://developers.openai.com/plugins/deploy/connect-chatgpt)

This makes an MCP-backed plugin the likely distribution package, but only once its
host supports the selected stateless protocol.

## Local route that preserves stateless MCP

The smallest current topology is:

```text
Codex conversation in ChatGPT desktop app
        |
        | local Streamable HTTP, stateless MCP 2026-07-28
        | mcp_2026_07_28 enabled in the Codex host
        v
127.0.0.1:3000/mcp
        |
        | validated local Aicadia-User-Id context
        v
World -> local PostgreSQL
```

Use a **user-level** MCP registration, not project-level configuration, so Aicadia
is available regardless of the folder opened in Codex. For this local single-user
loop, the host still needs the stable development UUID as an environment-backed
header. The UUID remains untrusted development context and must never become the
production identity model.

The feature flag and MCP registration solve protocol and reachability. They do not
by themselves reproduce the existing launcher's clean-room player envelope:

- an arbitrary folder can contribute `AGENTS.md`, skills and other tools;
- a normal Codex task may have shell, browser and source access;
- unrelated tools can create alternate sources of supposed live state; and
- technical progress may be visible in the conversation.

For a credible play conversation now, use a dedicated empty Aicadia player folder
or a dedicated host profile and enable only Aicadia. “Works from any folder” is an
installation property; it is not automatically a fail-closed-conformance property.
The current `tools/aicadia-agent` remains the strongest oracle for the latter.

## Hosted ChatGPT experiment without legacy fallback

If the goal is specifically ChatGPT Chat/Work rather than Codex in the desktop app,
the experiment must proceed as a strict gate:

1. Start the existing local server unchanged.
2. Make it remotely reachable with OpenAI Secure MCP Tunnel. A tunnel only forwards
   MCP traffic; it must not translate stateless requests into `initialize` sessions.
3. Create a developer-mode draft and scan the server.
4. Pass only if the observed hosted client calls `server/discover` and sends the
   required `2026-07-28` per-request metadata, imports exactly thirteen tools and
   receives the exact server instructions.
5. If the hosted client sends `initialize`, lacks stateless request metadata or
   cannot scan the catalog, stop. Record the host as unsupported; do not add a
   facade or compatibility path.

There is a practical uncertainty: the currently published `tunnel-client` guidance
and source examples are centered on the normal initialization lifecycle. The tunnel
may transparently relay newer requests, but that must be demonstrated end to end.
The hosted path is therefore a bounded compatibility spike, not yet an implementation
recommendation. [Secure MCP Tunnel](https://developers.openai.com/api/docs/guides/secure-mcp-tunnels)

## Identity and authorization

Protocol statelessness and user authentication are separate concerns. Production
must not expose a write-capable endpoint that trusts `Aicadia-User-Id` supplied by a
caller, prompt or model.

The intended production identity chain is:

```text
human signs in to Aicadia
  -> authorization server issues token for the Aicadia MCP resource
  -> Aicadia validates issuer, audience/resource, expiry, signature and scopes
  -> authenticated subject maps to exactly one durable Aicadia User
  -> MCP adapter passes that derived User identity to World
```

OAuth does not require a durable MCP transport session or a World session. Every
stateless request can carry and validate its bearer token independently. ChatGPT's
plugin auth guidance expects OAuth 2.1 protected-resource metadata, authorization
server discovery, resource binding, PKCE and server-side token validation.
[OpenAI plugin authentication](https://developers.openai.com/plugins/build/auth)

For a local single-user experiment, an outbound private tunnel may forward exactly
one fixed, validated local profile identity. That is a test fixture, not auth and
not evidence for multi-user safety.

## MCP versus a skill

The boundaries should remain:

| Concern | Owner |
|---|---|
| Reachability and transport | MCP host plus local/direct connection, tunnel or public HTTPS |
| Live World facts and actions | Aicadia MCP tools |
| Identity and authorization | OAuth resource server and Aicadia account mapping |
| Cross-tool player contract | `server/discover.instructions` |
| Operation-local rules | Tool descriptions, schemas and annotations |
| Deterministic validity and history | `World` |
| Optional activation/workflow convenience | Thin plugin skill |

A skill cannot make localhost reachable, enable a host feature, authenticate a
player or validate a World action. A thin `play-aicadia` skill may later help users
invoke Aicadia explicitly and reinforce that it is the sole live-game authority,
but it must refer to rather than duplicate the server-published contract.

The correct order is therefore **MCP connection first, observed Agent behavior
second, optional skill only if evaluations reveal a concrete activation or workflow
gap**.

## Folder independence

The connection should belong to the Agent host/account, not to the working tree:

- today, register local Aicadia in user-level Codex configuration shared by the
  ChatGPT desktop app and CLI;
- later, install/select the published Aicadia plugin in ChatGPT;
- for other Agents, configure the same hosted endpoint in their global/profile MCP
  settings; and
- never rely on a repo-local skill or `AGENTS.md` to discover the game service.

The hosted URL and authenticated identity make the caller's current folder
irrelevant. However, a host that also exposes repository files, shell or other tools
still needs to honor Aicadia's sole-authority and fail-closed contract. Folder
independence does not mean arbitrary folder instructions are safe.

## Alternatives

| Option | Stateless `2026-07-28` | Local now | Long-term fit | Verdict |
|---|---:|---:|---:|---|
| Existing isolated `tools/aicadia-agent` | Yes | Yes | Development oracle | Keep |
| ChatGPT desktop Codex + user-level MCP config + feature flag | Yes | Yes | Good local convenience; Codex-specific | **Recommended immediate experiment** |
| ChatGPT Chat/Work draft + Secure MCP Tunnel | Unknown until observed | Potentially | Good if hosted client supports current MCP | Run only as a strict compatibility spike |
| Legacy `initialize` facade | No | Technically possible | Violates confirmed direction and creates two protocols | Reject |
| Skill alone | N/A | No | Useful only as optional workflow packaging | Do not use as integration |
| Stable public MCP + OAuth + published plugin | Must require it | Not yet | Best distribution path once supported | **Production target** |
| Aicadia-owned Responses API chat client | Host-dependent | Could be built | More control, much more product/ops work | Defer |

## Exact evidence for the next step

Before changing Aicadia, run one token-bounded host compatibility experiment that
answers only this question:

> Can a Codex conversation in the ChatGPT desktop app, started outside the Aicadia
> repository with `mcp_2026_07_28` enabled, discover the exact current instructions
> and thirteen tools and complete the existing Character/onboarding flow exclusively
> over the local stateless MCP endpoint?

Success requires:

- no `initialize` or transport session;
- one successful `server/discover` carrying the exact current contract;
- the exact thirteen-tool catalog and annotations;
- the stable local profile User is verified without appearing as a tool argument;
- Character workshop and entry follow the existing confirmation contract;
- MCP failure causes no source, HTTP, database, browser or remembered-state fallback;
  and
- raw ids, protocol and tool progress stay out of the player-facing response.

This proves the desired local ChatGPT-desktop/Codex surface. A separate later test is
needed for hosted ChatGPT Chat/Work through Secure MCP Tunnel.

## Production end state

```text
ChatGPT plugin / Codex / another compatible MCP Agent
                           |
                           | stateless MCP 2026-07-28
                           | stable public HTTPS + OAuth bearer token
                           v
                 Aicadia MCP adapter
                 - validates token and scopes on every request
                 - derives Aicadia User from authenticated subject
                 - publishes server/discover instructions and tools
                           |
                           v
                    World interface
                           |
                           v
                       PostgreSQL
```

One endpoint and one protocol serve all compatible hosts. A host without current
stateless MCP support is simply unsupported until it catches up; Aicadia does not
retain a legacy path for it. A published plugin is the ChatGPT discovery and
installation package, not a second game API. Other Agents connect to the same URL
from any folder, and no server-side model call spends tokens for a player.

## Recommendation

Use the existing stateless Aicadia MCP unchanged. First prove it in a Codex
conversation inside the ChatGPT desktop app with the shared user-level MCP config
and `mcp_2026_07_28` enabled. Treat ChatGPT Chat/Work plus Secure MCP Tunnel as a
separate compatibility spike that must natively pass the same protocol. Add no
legacy facade; add a thin skill only after evidence shows a real workflow gap.

## Primary sources

- [Aicadia Agent interface](../game/agent-interface.md)
- [Aicadia local play](../game/local-play.md)
- [Aicadia local Codex adapter](../../tools/aicadia-agent)
- [Aicadia current MCP Agent guidance](current-mcp-agent-guidance.md)
- [Aicadia server implementation](../../src/server.rs)
- [OpenAI: Codex MCP configuration shared by desktop app and CLI](https://learn.chatgpt.com/docs/extend/mcp)
- [OpenAI: ChatGPT Developer mode](https://developers.openai.com/api/docs/guides/developer-mode)
- [OpenAI: plugins](https://learn.chatgpt.com/docs/plugins)
- [OpenAI: connect and test a plugin](https://developers.openai.com/plugins/deploy/connect-chatgpt)
- [OpenAI: Secure MCP Tunnel](https://developers.openai.com/api/docs/guides/secure-mcp-tunnels)
- [OpenAI: authenticate plugin users](https://developers.openai.com/plugins/build/auth)
- [OpenAI: plugin architecture](https://developers.openai.com/plugins/concepts/plugins)
- [OpenAI: skills](https://developers.openai.com/plugins/concepts/skills)
- [MCP `2026-07-28` specification release](https://blog.modelcontextprotocol.io/posts/2026-07-28/)
- [MCP `2026-07-28` discovery](https://modelcontextprotocol.io/specification/2026-07-28/server/discover)
- [MCP `2026-07-28` Streamable HTTP](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http)
- [MCP `2026-07-28` authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization)
