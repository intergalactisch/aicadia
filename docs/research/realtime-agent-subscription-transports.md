# Realtime Agent subscription transports

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, evidence, inferences and candidate
> implications.
> **Excludes:** product decisions and current implementation contracts; see
> `docs/game/`.

Date: 2026-08-16

Status: complete transport and host-compatibility research; no subscription,
resource, polling, webhook or socket behavior below is accepted Aicadia behavior

## Question and evidence boundary

What is the smallest external transport through which an active Agent host can be
told that a subscribed Entity, Place or other bounded World representation changed?
How do current MCP `subscriptions/listen`, raw Server-Sent Events, WebSocket,
polling and webhooks differ in correctness, reconnect, replay, ordering,
backpressure, authentication, host support and ability to reach an LLM?

This report uses current protocol specifications, standards, official vendor
documentation, the official Rust MCP SDK and a read-only audit of this checkout and
the installed Codex host. It narrows, but does not repeat, the earlier
[MCP subscriptions and collective Agent intents](mcp-subscriptions-and-collective-agent-intents.md)
report.

Every material statement is labelled as one of:

- **Evidence:** directly established by a cited specification, official
  documentation, source or local inspection;
- **Inference:** analysis of that evidence; or
- **Candidate implication:** an unaccepted direction that still requires product
  choice, an accepted plan and exact evidence.

No live Agent host or LLM was invoked. No Codex, ChatGPT or Claude model turn was
tested. Host documentation, protocol capability, SDK capability, configured
transport and observed host behavior are separate claims.

## Result in one sentence

**Inference.** The widest portable Agent capability today is an explicit User turn
followed by one authoritative bounded MCP read or tool call. Current MCP resource
subscriptions are the least-ceremonial *optional accelerator* for a host that
actually implements them, but neither ChatGPT nor Codex publicly guarantees that
host behavior, and no push transport may be required for correctness or described
as an Agent having perceived or understood a change.

**Candidate implication.** If Aicadia experiments, keep one semantic invalidation
contract: an exact World representation may be stale, so refetch current
authoritative state. Carry that hint through MCP `subscriptions/listen` for a
proven compatible host; use the same ordinary read on the next explicit User turn
when the host does not listen, disconnected or dropped every hint. Do not add a raw
SSE API, WebSocket protocol, webhook registry or background model invocation first.

## The four layers that “subscribe an Agent” can hide

| Layer | What it can establish | What it cannot establish |
|---|---|---|
| World state | Current durable Entity, Place and Activity facts | That any host or Agent saw them |
| Server-to-host transport | A connected host received a small change hint | Current truth, replay, perception or model execution |
| Host refresh and presentation | The host refetched state, invalidated a cache or showed something | That an LLM received or understood it |
| Explicit model turn | The host supplied selected current material to an invoked Agent | That the Agent's interpretation is World truth |

**Evidence.** MCP deliberately does not prescribe how an AI application uses an LLM
or supplied context. Resources are application-driven: the host decides how to
select, cache, display or include them. MCP notifications are JSON-RPC messages that
expect no response. The explicit MCP operation that asks a client for model output
is sampling, and in `2026-07-28` it is deprecated and nested within an already-active
multi-round-trip request rather than available from a subscription stream.
[MCP architecture](https://modelcontextprotocol.io/docs/2026-07-28/learn/architecture),
[resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources),
[sampling](https://modelcontextprotocol.io/specification/2026-07-28/client/sampling),
[multi-round-trip requests](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/mrtr)

**Inference.** SSE, WebSocket and webhook delivery can run code in a host process.
None is itself an LLM invocation. Any host that converts an inbound event into a
model turn is adding host policy, token spend, prompt-injection exposure and an
availability boundary beyond the transport.

**Candidate implication.** Aicadia should name evidence precisely:

- `World committed` means durable state and Activity were accepted;
- `host notified` means only that a connected transport accepted a hint;
- `host refreshed` means an authorized current read completed; and
- `Agent processed` requires evidence from one explicit User-owned model turn.

## Transport comparison

| Option | Server-to-host shape | Reconnect and replay | Flow control and ordering | Generic Agent-host fit | KISS/Terry fit |
|---|---|---|---|---|---|
| MCP `subscriptions/listen` over Streamable HTTP | One long-lived POST response encoded as SSE; exact resource URI invalidation | Re-listen after disconnect; no MCP session, `Last-Event-ID` or missed-message replay | SDK may bound buffers; no application acknowledgment or cross-stream World order | Standard protocol, but only when the host implements and uses this optional feature | Best candidate accelerator; authoritative refetch remains mandatory |
| Raw SSE / browser `EventSource` | One-way HTTP `text/event-stream`, normally browser GET | Browser reconnect and `Last-Event-ID` exist; replay requires Aicadia to retain and interpret event IDs | Stream order only; no application acknowledgment; slow-client policy is application work | Requires a custom Aicadia-aware host integration | Duplicates MCP transport and invents another public contract |
| WebSocket | One upgraded, bidirectional framed TCP connection | Reconnect, deduplication and replay are entirely application protocols | Ordered frames on one connection; buffering, acknowledgments and slow consumers are application work | Requires a custom host and protocol | Full duplex solves no current need because writes already use MCP requests |
| Conditional polling | Repeated current-state reads; HTTP can use validators such as ETag | No missed-hint problem; every successful read obtains a current representation | Caller naturally controls request rate; latency and load follow interval | Explicit-turn reads are portable; autonomous background polling is host-specific | Correct fallback, not a second truth; do not poll from World to invoke Agents |
| Webhook | Aicadia POSTs to a registered callback endpoint | Retry, duplicate and retention policy must be designed per sender | Receiver acknowledges each delivery; sender still needs queues, signing and deduplication | Requires a reachable, authenticated callback owned by the User or host | Too much registration/security/retry ceremony for local and generic Agent hosts |

### 1. MCP resource subscriptions

**Evidence.** In MCP `2026-07-28`, a client opens `subscriptions/listen` and requests
an explicit filter. The core filter contains tool-, prompt- and resource-list change
categories plus `resourceSubscriptions`, which is a list of resource URI strings.
The server acknowledges the accepted subset before delivering notifications and
must not deliver an unrequested category. A resource update notification contains
the URI whose representation changed, not its new contents. The documented flow is
notification followed by `resources/read`.
[MCP subscriptions](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/subscriptions),
[normative schema](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/2026-07-28/schema.ts#L1261-L1401),
[MCP resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources)

**Evidence.** Under current Streamable HTTP, each request is a POST. A listen
response is a long-lived SSE stream. `2026-07-28` has no `Mcp-Session-Id`, standalone
GET stream or resumable SSE via `Last-Event-ID`; servers must ignore those legacy
headers. Keep-alive SSE comments are encouraged and closing the response cancels the
listen request. After reconnect, the client must establish a new subscription; the
server retains no subscription state across reconnections.
[MCP Streamable HTTP](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http),
[MCP subscriptions](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/subscriptions)

**Inference.** MCP supplies an invalidation wire, not a durable event log. A client
can miss every notification while disconnected and still recover by re-listening
and reading current state. Aicadia does not need delivery exactly once because one
or many identical hints have the same meaning: the representation may be stale.

**Inference.** The protocol filter contains exact URI strings, not patterns or
spatial predicates. A client cannot subscribe to `entity://*` or “everything near
this Character” through the core filter. Dynamic relevance must be represented by
known exact URIs or by a stable exact index/view whose contents change.

**Candidate implication.** A safe host sequence is:

1. request an exact bounded subscription and await acknowledgment;
2. read the exact current representation;
3. coalesce any number of update hints into “stale”;
4. perform one authorized bounded refetch before the next dependent action;
5. after abrupt disconnect, re-listen and refetch before trusting cached state; and
6. carry an observed revision into a mutation so World can reject stale work.

That sequence makes notification loss harmless. It does not establish how a commit
reaches every server instance holding an open listen stream; database-to-process
fan-out is a separate internal transport question.

### 2. Raw SSE and `EventSource`

**Evidence.** The HTML standard defines `EventSource` as a receiving interface for
`text/event-stream`. A user agent reconnects after interruption, carries the last
event ID in `Last-Event-ID` when non-empty, accepts a server-supplied retry delay and
can be told to stop reconnecting with HTTP `204 No Content`.
[WHATWG Server-Sent Events](https://html.spec.whatwg.org/multipage/server-sent-events.html)

**Inference.** `Last-Event-ID` transports an opaque identifier; it does not require
the server to retain events or replay them. Durable replay, retention, authorization
across reconnect, compaction and cursor expiry would all be Aicadia protocol work.

**Inference.** Raw SSE is attractive for a custom browser because it is one-way and
ordinary HTTP. For Agent hosts it loses the principal benefit of MCP: the host no
longer knows that the message is a standard resource invalidation followed by a
standard resource read. A separate raw SSE endpoint would need its own payload,
identity, authorization, reconnect, versioning and host adapter while current MCP
already uses SSE framing.

**Candidate implication.** Do not expose raw SSE as a second Agent API merely to
obtain browser auto-reconnect. If a future read-only Aicadia browser needs live
invalidation, it may consume the same internal hint through a browser-specific SSE
adapter, but that would not prove any Agent host or LLM behavior.

### 3. WebSocket

**Evidence.** RFC 6455 defines an HTTP Upgrade handshake followed by bidirectional
message framing over a persistent connection, including fragmentation, control
frames and connection close behavior. It does not define an application resource
model, subscription filter, event ID, replay log, idempotency rule or authorization
semantics for World changes.
[RFC 6455](https://www.rfc-editor.org/rfc/rfc6455.html)

**Inference.** WebSocket's extra value is simultaneous client-to-server traffic on
the same connection. Aicadia already has normal stateless MCP requests for reads and
writes, while its desired push is a small server-to-host invalidation. Choosing
WebSocket would require inventing the exact semantics MCP already standardizes and
would bind Aicadia to custom hosts.

**Candidate implication.** WebSocket earns a place only after one accepted current
behavior needs high-frequency bidirectional messages that cannot use ordinary MCP
requests plus one-way invalidation. “Realtime” alone does not establish that need.

### 4. Polling and explicit-turn pull

**Evidence.** HTTP conditional requests can send an entity tag through
`If-None-Match`; when the selected representation has not changed, a server can
return `304 Not Modified` instead of the representation body. HTTP defines the cache
validator and response semantics, not an Agent timer or model invocation.
[RFC 9110: If-None-Match](https://www.rfc-editor.org/rfc/rfc9110.html#name-if-none-match),
[RFC 9110: 304 Not Modified](https://www.rfc-editor.org/rfc/rfc9110.html#name-304-not-modified)

**Inference.** Direct HTTP polling is not an acceptable alternate World authority
for Aicadia's Agent: current project rules make MCP the sole live-game authority.
The corresponding portable behavior is a bounded MCP read or tool call during an
explicit User turn. A custom host could autonomously poll the same MCP resource, but
generic hosts do not promise that timer.

**Inference.** Polling current state cannot miss correctness-relevant changes in the
same way an ephemeral hint can: the successful read returns the current durable
representation. Its trade-off is latency and load. One poll per User per Entity at a
short fixed interval is not a million-User design; explicit-turn reads, bounded
interest, conditional/cache-aware host behavior and optional invalidation reduce
that pressure.

**Candidate implication.** Treat pull as the correctness path and push as latency
reduction. This is one semantic system, not two: both end in the same authoritative
read and no mutation trusts a notification payload.

### 5. Webhooks

**Evidence.** A concrete first-party webhook implementation illustrates the work a
webhook introduces: GitHub recommends subscribing only to needed event types, HTTPS,
a secret and signature validation, a prompt `2xx` response and duplicate-safe
processing. GitHub does not automatically redeliver failed deliveries; consumers
must arrange manual or scripted redelivery. These are GitHub's policies, not a
universal webhook standard.
[GitHub webhook best practices](https://docs.github.com/en/webhooks/using-webhooks/best-practices-for-using-webhooks),
[signature validation](https://docs.github.com/en/webhooks/using-webhooks/validating-webhook-deliveries),
[failed deliveries](https://docs.github.com/en/webhooks/using-webhooks/handling-failed-webhook-deliveries)

**Inference.** An Aicadia webhook would need a durable or renewable callback
registration, ownership proof, public reachability or a tunnel, endpoint secrets,
signature rotation, event IDs, retry/expiry policy, rate limits, revocation and
deduplication. A local Agent host behind NAT generally cannot receive it directly.
Even a successful callback reaches user-owned software, not necessarily the Agent
host or model.

**Candidate implication.** Webhooks may later suit an explicitly accepted
server-to-server integration. They are a poor first subscription mechanism for an
open population of local and hosted Agent apps.

### 6. WebTransport

**Inference.** WebTransport is not concretely relevant to the current problem. No
inspected Aicadia host surface or MCP `2026-07-28` transport uses it, and the desired
message is a small reliable invalidation rather than multiplexed datagrams or
independent streams. Adding it would require a custom host and protocol like
WebSocket, with less present ecosystem evidence.

## Actual Agent-host compatibility

The matrix separates five questions that are often collapsed into “supports MCP”:

1. Can the host call tools?
2. Can it list/read ordinary runtime resources?
3. Does it implement current `subscriptions/listen`, not merely an older SSE
   transport or `list_changed` notification?
4. Does it keep and recover a long-lived stream and surface an update to the User?
5. Can an inbound event start a model turn, and if so, is that portable and wanted?

`Yes` below means official documentation or local evidence establishes that exact
claim. `Unknown` means it must be tested and may not be assumed.

| Host surface | MCP tools | Ordinary runtime resources | Catalog `list_changed` | Exact `subscriptions/listen` / resource update | Stream recovery and User surfacing | Automatic model invocation |
|---|---|---|---|---|---|---|
| Codex host in ChatGPT desktop, Codex CLI and IDE extension | **Yes.** OpenAI documents direct configured MCP servers; current local Aicadia tool discovery works | **Unknown.** Current official Codex MCP feature list does not claim generic resource list/read UX | **Unknown** | **Unknown.** Current protocol mode does not prove the host opens listen | Streamable HTTP is supported; listen lifecycle and presentation are **unknown** | **No documented guarantee; never assume** |
| ChatGPT web / Work plugin | **Yes.** Developer mode scans, lists and calls MCP tools | Generic live Entity resources are **unknown**; skill resources are import-time snapshots and UI resources are special-purpose | **Unknown.** Metadata refresh is documented as a manual rescan/snapshot flow | **Unknown; no official guarantee found** | Remote Streamable HTTP is required, but long-lived resource-listen behavior and surfacing are **unknown** | **No documented notification-to-model path; never assume** |
| Claude Code core MCP | **Yes** | **Yes.** Resources appear through `@`, are fetched when referenced, and list/read tools are supplied | **Yes.** Claude Code refreshes tool, prompt and resource catalogs and documents reconnect/backoff | **Unknown.** Official docs do not claim current exact resource `subscriptions/listen`; `list_changed` is not resource-content invalidation | HTTP/SSE reconnect is documented; core Entity-update presentation remains **unknown** | **No core resource-update guarantee; never assume** |
| Claude Code `claude/channel` extension | Tools may accompany the channel | Not the mechanism | Not the mechanism | **No.** It uses vendor methods such as `notifications/claude/channel` | Local stdio bridge, opt-in flags and visible session injection are documented | **Yes, it can make Claude react, but only as an Anthropic research-preview extension** |
| Future or other Agent app | Only if implemented | Only if implemented | Only if implemented | Only if it implements `2026-07-28` and opts in | Unknown until one pinned host smoke | Unknown; protocol support never proves invocation policy |

### Codex and ChatGPT evidence

**Evidence.** OpenAI documents that the Codex host in the ChatGPT desktop app,
Codex CLI and the IDE extension share MCP configuration and support stdio and
Streamable HTTP servers, bearer/OAuth authentication and server instructions. The
page's stated supported server features do not claim generic runtime resources,
resource subscriptions or notification presentation.
[OpenAI Codex MCP documentation](https://learn.chatgpt.com/docs/extend/mcp)

**Evidence.** The installed local host is `codex-cli 0.147.0`. Its read-only feature
list reports `mcp_2026_07_28` enabled and under development. Existing Aicadia
evidence establishes current tool discovery through that mode, not resources or
listen. No live subscription smoke exists in this checkout.

**Evidence.** Current ChatGPT plugin guidance is tools-first: developer mode connects
to a public or tunneled Streamable HTTP endpoint, discovers tools and metadata, and
tests model tool selection. Metadata changes require refresh. MCP-hosted skill
resources are explicitly submission-time snapshots rather than live runtime
resources. The current guidance does not promise `subscriptions/listen`, resource
update surfacing or notification-triggered model execution.
[Connect and test a plugin](https://developers.openai.com/plugins/deploy/connect-chatgpt),
[Build an MCP server](https://developers.openai.com/plugins/build/mcp-server)

**Inference.** OpenAI transport support is not evidence of OpenAI subscription
behavior. “It can connect to a Streamable HTTP endpoint” proves requests can reach
the server, not that the host opens a long-lived listen request, rereads an updated
Entity, shows a badge or starts a model turn.

**Candidate implication.** Aicadia must keep a normal explicit-turn read usable in
Codex and ChatGPT. Before claiming realtime support for either, run one direct smoke
against a disposable World that proves all of: acknowledged exact subscription,
commit after baseline, received notification, authoritative refetch, reconnect
refetch and zero automatic model calls.

### Claude Code evidence and the vendor-extension trap

**Evidence.** Claude Code documents tools, ordinary MCP resources referenced through
`@`, automatic tools for resource list/read, dynamic `list_changed` refresh for
tools/prompts/resources and automatic reconnect with bounded exponential backoff for
HTTP and SSE transports. Its documentation does not claim current exact-resource
`subscriptions/listen` behavior.
[Claude Code MCP](https://code.claude.com/docs/en/mcp)

**Evidence.** Claude Code separately documents channels as research-preview push.
A local stdio MCP server declares the vendor capability `claude/channel` and emits
`notifications/claude/channel`; the User must opt in through channel/development
flags. The event is injected into the session and the documentation explicitly says
Claude can react and start responding. The method and schema are Claude Code
extensions, not core MCP.
[Claude Code channels reference](https://code.claude.com/docs/en/channels-reference)

**Inference.** Claude channels prove that a host *can* deliberately turn external
push into model activity. They do not prove that core MCP resource notifications do
so, and adopting them would make Aicadia provider-specific while permitting
unprompted token spend and hostile external text to enter a session. That directly
crosses Aicadia's current Agent-intelligence and no-unconscious-token-burn boundary.

**Candidate implication.** Do not use `claude/channel` as the Aicadia subscription
contract. It may be useful only as comparative lab evidence of host presentation,
clearly labelled vendor-specific and never counted as portable MCP or production
authorization.

## Global board versus exact Entity or Place resources

This section compares only what the transports can express; it does not choose a
World model or deliberation mechanic.

| Resource shape | Transport advantage | Transport cost or failure |
|---|---|---|
| One global World board URI | Every compatible host can know one exact URI in advance | Every change invalidates every subscriber; one hot fan-out and broad URI visibility defeat locality |
| Exact Entity and Place URIs | Core MCP's exact URI filter maps directly; fan-out remains subject-local | Host must already know each URI and replace subscriptions as interest changes; no wildcard discovers new relevant subjects |
| One stable per-Character or per-current-context URI | One exact subscription can invalidate a bounded relevant view and hide dynamic URI churn from limited hosts | Constructing and authorizing that view is additional World/host design; it must not make World infer semantic relevance |

**Inference.** The global board is superficially simplest but creates the wrong
million-User hot path: unrelated World mutations wake every connected subscriber.
Exact Entity/Place subscriptions fit the protocol and scale boundary better but are
not portable until target hosts actually implement listen. A stable bounded context
resource can bridge exact-only transport, but its contents and ownership are a game
decision rather than a transport fact.

**Candidate implication.** Do not use `resourcesListChanged` as an Entity-change
broadcast. It means the visible resource catalog changed, not that every Entity's
contents changed. Do not dynamically mint a proposal URI and expect unaware clients
to subscribe: core MCP has no wildcard. A known exact Entity, Place or bounded
context/index URI must be the discovery seam if listen is used.

## Backpressure, hot Entities and millions of connections

**Evidence.** Official `rmcp` `3.1.1`, already pinned by Aicadia, supplies
server-side accepted filters and a filter-enforcing `SubscriptionSink`, plus client
`listen()` support. Its default client channel holds 64 notifications. If the
consumer does not drain the channel before it fills, the SDK reports a lagged
subscription and cancels it; an abrupt close is not resumable and requires another
listen call.
[`rmcp` 3.1.1 server subscriptions](https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.1.1/crates/rmcp/src/service/server.rs),
[`rmcp` 3.1.1 client subscriptions](https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.1.1/crates/rmcp/src/service/client.rs)

**Inference.** The bounded buffer proves slow consumers are expected; it does not
prove the server can hold millions of streams. One Entity watched by millions still
requires millions of transient deliveries and may cause a refetch stampede. One
notification per accepted mutation per connected recipient is an operational fan-out
cost even when no recipient row is stored.

**Candidate implication.** A scalable invalidation experiment must prove:

- hints can be duplicated, dropped and coalesced to one stale bit;
- one slow host is disconnected or resynchronized without blocking others;
- notification payloads contain no private Entity state;
- refetch is authorized and bounded independently of listen acknowledgment;
- reconnect performs baseline read rather than replaying an unbounded backlog;
- a hot Entity does not create one durable row per observer or one global lock; and
- host refetches use jitter/coalescing or another bounded admission rule before a
  million-subscriber stampede can hit the same read.

These are required properties, not evidence that Postgres, one Rust process or the
current SDK already provides production fan-out capacity.

## Authentication and privacy

**Evidence.** Current MCP authorization for protected HTTP servers uses bearer
tokens on HTTP requests with resource-server validation and audience binding.
Streamable HTTP requires Origin validation and recommends authentication. Resource
visibility may depend on authorization, and private cache entries must not cross
authorization contexts.
[MCP authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization),
[authorization security](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization/security-considerations),
[Streamable HTTP security](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http#security--endpoint),
[MCP caching](https://modelcontextprotocol.io/specification/2026-07-28/server/utilities/caching)

**Inference.** A resource URI can itself disclose that an Entity, Place or dispute
exists. Accepting an exact URI in a filter is not authority to read it forever.
Every baseline, refetch and mutation must reauthorize against current durable
context. The game must separately decide whether later loss of access suppresses
hints, closes a stream or merely makes refetch fail.

**Candidate implication.** Hosted subscription work cannot precede a real identity
and authorization contract. A guessed URI, successful listen acknowledgment or
prior Place presence must never become continuing Entity visibility.

## Current Aicadia fit

**Evidence.** Aicadia pins `rmcp = 3.1.1` with server and Streamable HTTP support. It
advertises only the tool capability and fifteen fixed tools. It implements neither
resources nor `accepted_subscription_filter`; the SDK default therefore leaves
`subscriptions/listen` unimplemented. The server uses stateless HTTP and a fresh
handler per request. Simple responses prefer JSON, while a future listen request
could still use SSE.
[Aicadia MCP handler](../../src/server/mcp.rs),
[Aicadia server transport](../../src/server/mod.rs),
[Aicadia dependency](../../Cargo.toml)

**Evidence.** Current transport configuration disables SSE keep-alive comments and
retry hints. The endpoint binds only to loopback, validates one local Origin and uses
`Aicadia-User-Id` as untrusted development context rather than authentication.
[Aicadia server transport](../../src/server/mod.rs),
[protocol request context](../game/protocol.md#request-context)

**Inference.** The current library is sufficient for a bounded local MCP
subscription experiment. Current Aicadia behavior, authorization, host support and
massive fan-out are not. A hosted listen stream would also need explicit keep-alive,
proxy buffering and timeout evidence; the present loopback configuration cannot be
promoted by assumption.

**Candidate implication.** The smallest honest experiment is not a production event
system. It is one disposable Entity-like resource, one pinned host, one exact
subscription, one durable state change, one content-free update, one authoritative
readback, one forced disconnect and one reconnect read. The verdict must name the
host and version and must report `model_calls = 0` unless the User separately
authorizes one explicit Agent turn.

## Smallest candidate architecture, not a decision

```text
explicit User turn or compatible active host
        |
        | 1. normal authorized bounded MCP read (always works)
        v
durable current Entity / Place representation

World transaction commits a later change
        |
        | 2. transient content-free "this exact URI may be stale"
        |    only to connected, authorized, opted-in hosts
        v
MCP subscriptions/listen stream (optional acceleration)
        |
        | 3. coalesce; no model call
        v
host marks representation stale
        |
        | 4. on explicit use, authorized refetch + observed revision
        v
World validates current state before any mutation
```

**Inference.** This is one semantic system because both the no-push and push paths
end in the same bounded authoritative read. Replacing the optional transport later
does not change World correctness. Raw SSE, WebSocket, webhook delivery and durable
replay are absent until a concrete current host or game behavior proves why MCP
invalidation plus pull is insufficient.

## What is and is not established

### Established

- MCP `2026-07-28` standardizes opt-in exact-resource invalidation on a long-lived
  stream followed by authoritative refetch.
- Current MCP Streamable HTTP subscriptions are SSE-framed but have no protocol
  session, `Last-Event-ID` resume or replay.
- Raw EventSource offers reconnect and last-event-ID transport, but replay remains
  an application responsibility.
- WebSocket supplies bidirectional framing, not World subscription semantics.
- Polling or explicit-turn pull can always recover current state; automatic host
  polling and model invocation are separate capabilities.
- Webhooks add registration, reachability, signing, retry and deduplication work and
  do not directly reach generic local Agent hosts.
- Claude Code documents tools, resources, catalog refresh and reconnect, but not
  current exact-resource `subscriptions/listen`.
- Claude Code's automatic external-event reaction uses a vendor-specific,
  research-preview channel extension and is not portable core MCP.
- OpenAI documents Streamable HTTP and MCP tools for current Codex and ChatGPT
  surfaces, but does not currently guarantee exact resource listen, update
  presentation or notification-triggered model work.
- Aicadia's Rust SDK can implement a local subscription experiment; current Aicadia
  advertises tools only and has no resource, listen, authentication or hosted fan-out
  behavior.

### Not established

- that Codex, ChatGPT desktop, ChatGPT web or Work currently opens
  `subscriptions/listen` for an Aicadia resource;
- that Claude Code implements the `2026-07-28` exact-resource listen operation;
- that any core resource update is shown to a User, inserted into context or causes
  a model turn;
- that one long-lived stream survives mobile suspension, process sleep, proxy
  timeout or server deployment;
- that the current loopback server can authenticate or privately fan out hosted
  Entity updates;
- that a global board, exact Entity/Place set or per-Character context resource is
  the right game design;
- that `rmcp`, Postgres or one Aicadia process supports millions of concurrent
  streams or one Entity watched by millions;
- that push reduces total read load under a hot-subject refetch stampede; or
- that Aicadia needs durable replay, WebSocket, webhook or WebTransport at all.

## Primary sources

- Model Context Protocol,
  [`2026-07-28` subscriptions](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/subscriptions)
- Model Context Protocol,
  [`2026-07-28` Streamable HTTP](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http)
- Model Context Protocol,
  [`2026-07-28` resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources)
- Model Context Protocol,
  [`2026-07-28` authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization)
- WHATWG,
  [Server-Sent Events](https://html.spec.whatwg.org/multipage/server-sent-events.html)
- IETF,
  [RFC 6455: The WebSocket Protocol](https://www.rfc-editor.org/rfc/rfc6455.html)
- IETF,
  [RFC 9110: HTTP Semantics](https://www.rfc-editor.org/rfc/rfc9110.html)
- OpenAI,
  [Codex MCP](https://learn.chatgpt.com/docs/extend/mcp)
- OpenAI,
  [Connect and test a plugin](https://developers.openai.com/plugins/deploy/connect-chatgpt)
- OpenAI,
  [Build an MCP server](https://developers.openai.com/plugins/build/mcp-server)
- Anthropic,
  [Claude Code MCP](https://code.claude.com/docs/en/mcp)
- Anthropic,
  [Claude Code channels reference](https://code.claude.com/docs/en/channels-reference)
- GitHub,
  [Webhook best practices](https://docs.github.com/en/webhooks/using-webhooks/best-practices-for-using-webhooks)
- Model Context Protocol official Rust SDK,
  [`rmcp` 3.1.1](https://github.com/modelcontextprotocol/rust-sdk/tree/rmcp-v3.1.1)
