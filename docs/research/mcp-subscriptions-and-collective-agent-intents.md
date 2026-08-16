# MCP subscriptions and collective Agent intents

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, evidence, inferences and candidate implications.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Date: 2026-08-16

Status: complete research; no proposal, voting, subscription or runtime design below
is accepted Aicadia behavior

## Question and evidence boundary

Can current MCP listeners, subscriptions and notifications support a game mechanic in
which explicitly invoked, nearby Agents notice a proposed World change, refetch one
shared proposal board, deliberate, submit candidate outcomes or votes, and eventually
produce a bounded change package—while World remains authoritative but never performs
semantic inference?

This report uses the published MCP `2026-07-28` specification, official SDK material
and a read-only audit of the current Aicadia checkout. **Evidence** is directly
supported by a cited primary source. **Inference** is analysis of that evidence.
**Candidate implication** is an unaccepted direction that still requires a product
choice, an accepted plan and exact evidence.

No live MCP host was tested. In particular, this report does not establish how Codex,
ChatGPT, Claude, VS Code or any other host currently presents a resource notification,
whether it keeps a subscription open while idle, or whether it can activate a model
turn. Protocol support, SDK support, host integration and desired Aicadia gameplay are
four different claims.

## Result in one sentence

**Inference.** MCP can carry lossy, opt-in invalidation hints to an already connected
client; it cannot portably wake a stopped host, invoke an Agent, spend model tokens,
decide semantic consequences, run deliberation or settle concurrent World state.

**Candidate implication.** The promising seam is a durable, authorization-scoped
proposal-board read plus explicit Agent-authored write tools. A
`notifications/resources/updated` message may mark that read stale; every meaningful
Agent action starts only in an explicit User-owned turn, refetches authoritative state,
and submits one bounded typed request. World validates structure, eligibility,
versions, bounds, idempotency and atomic history, never the meaning of an explosion,
a tree, a Property name or prose.

## What MCP `2026-07-28` actually provides

### `subscriptions/listen` is one opt-in change stream

**Evidence.** A client opens a long-lived `subscriptions/listen` request and names the
notification categories it wants. The standard filter contains exactly:

- `toolsListChanged`;
- `promptsListChanged`;
- `resourcesListChanged`; and
- `resourceSubscriptions`, a list of exact resource URIs.

The server must acknowledge the accepted subset before sending matching messages,
must tag each message with the listen request id, and must not send a notification
type the client did not request. Multiple listen requests may coexist.
[MCP subscriptions](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/subscriptions),
[normative schema](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/2026-07-28/schema.ts#L1261-L1401)

**Evidence.** `notifications/resources/updated` contains a resource URI, not the new
resource body. Its meaning is that the resource changed and may need to be read again.
The resource specification's own flow is notification followed by `resources/read`.
Resources are application-driven: the host decides whether and how to display,
select, cache or insert them into model context.
[MCP resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources)

**Inference.** This is already the right wire shape for a stale proposal-board hint:
small, content-free and followed by authoritative refetch. It is not a general event
bus. Current core MCP has no standard `proposal/opened`, `vote/requested`,
`entity/changed` or `agent/wake` notification category.

**Candidate implication.** If Aicadia uses core MCP rather than an Aicadia-specific
extension, an Entity-, Place- or deliberation-scoped proposal board can be represented
as a resource URI. The notification says only “the representation behind this URI may
have changed.” Board contents and every write remain normal bounded reads and tool
calls over the same World authority.

### Resource notifications invalidate; they do not establish truth

**Evidence.** Cacheable MCP results carry `ttlMs` and `cacheScope`. A relevant change
notification invalidates a fresh cached response. `private` responses may be reused
only within the same authorization context; access controls still must be enforced by
the server. TTL is a freshness hint, not a guarantee, and clients may serve stale data
after refetch failure.
[MCP caching](https://modelcontextprotocol.io/specification/2026-07-28/server/utilities/caching)

**Inference.** A notification cannot be the authoritative proposal, deadline,
eligibility set, argument, vote, outcome or World mutation. Even a connected client
can be behind, can drop the stream, can lag its buffer, or can choose not to refetch.
The current resource representation—and ultimately World state—must answer every
correctness question.

**Candidate implication.** A proposal-board read should expose a current revision or
other explicit freshness token and return `cacheScope: private` whenever its contents
or visibility depend on the User. A subsequent proposal, position or settlement call
should carry the relevant observed revision and fail closed when the durable board or
affected World facts have changed. Exact wire fields remain a design question.

### Progress and logging are the wrong channels

**Evidence.** `notifications/progress` is allowed only for a token supplied on one
active request, must stop after that request completes and is optional even when the
client supplied the token.
[MCP progress](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/progress)

**Evidence.** `notifications/message` is request-scoped logging. The client must opt in
with a log level on that request; the message cannot be delivered on a
`subscriptions/listen` stream. Logging is deprecated in `2026-07-28`, and new
implementations should use ordinary observability instead.
[MCP logging](https://modelcontextprotocol.io/specification/2026-07-28/server/utilities/logging)

**Inference.** Neither mechanism can announce a proposal that is opened after some
unrelated tool call has finished. Encoding game events as progress or logs would make
domain state ephemeral, unauthoritative and host-dependent.

### Notification, host activity and LLM invocation are distinct

**Evidence.** MCP explicitly limits itself to context exchange and does not dictate
how an AI application uses LLMs or provided context. The MCP host owns the MCP client;
notifications are JSON-RPC messages that expect no response. Resources are
application-driven.
[MCP architecture](https://modelcontextprotocol.io/docs/2026-07-28/learn/architecture),
[MCP resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources)

**Evidence.** The protocol feature that actually requests a model generation is
`sampling/createMessage`. In `2026-07-28` it can occur only as an input request nested
inside the multi-round-trip response to an already active `tools/call`, `prompts/get`
or `resources/read`. Sampling is deprecated, requires a declared client capability,
and its specification recommends human review. The server must not assume the client
will fulfill any input request or retry.
[MCP sampling](https://modelcontextprotocol.io/specification/2026-07-28/client/sampling),
[MCP multi round-trip requests](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/mrtr)

**Inference.** An SSE message can run a network callback in an MCP client that is
already alive and listening. The specification provides no operating-system push,
host wake-up, scheduled Agent turn, conversation injection or automatic token spend.
A host may choose to surface a badge, invalidate a cache or start a model turn, but
that is host-specific policy and not portable MCP behavior.

**Candidate implication.** Aicadia may safely treat a notification as a transport
hint to the host. It must never describe that hint as “the nearby Agent heard the
proposal” until the User explicitly invokes the Agent and the Agent refetches it. This
preserves the current no-unconscious-token-burn boundary and makes non-subscribing
hosts functionally correct, merely less timely.

**Inference.** MCP has no client-to-client or Agent-to-Agent conversation channel.
Agents can “talk” only indirectly: one explicit Agent call stores an authorized
contribution in World, another later explicit Agent call reads that durable
contribution and stores a response. A short deliberation window therefore limits who
can participate to Users whose hosts are connected, notice the hint and explicitly
invoke their Agents in time. A subscription alone cannot fill the room with automatic
model turns.

## Streamable HTTP lifecycle and loss model

**Evidence.** Current Streamable HTTP uses one POST endpoint. A normal request may
return JSON or a request-scoped SSE response. A `subscriptions/listen` POST returns a
long-lived SSE response. The standalone GET stream and protocol-level sessions were
removed in `2026-07-28`; there is no `Mcp-Session-Id` and no `Last-Event-ID` resume.
Unexpected disconnect permits reconnect, but subscription state and missed
notifications are not replayed. Keep-alive comment lines are encouraged for long-lived
streams, and closing the HTTP response stream cancels the listen request.
[MCP Streamable HTTP](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http),
[MCP subscriptions](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/subscriptions)

**Inference.** At-least-once, exactly-once and ordered delivery across reconnects are
not MCP guarantees. A notification can be missed between disconnect and re-listen.
Protocol session removal makes ordinary requests easy to route to any World instance,
but the one open SSE response still terminates at one serving instance for its
lifetime.

**Candidate implication.** A correct client-side invalidation sequence is:

1. open `subscriptions/listen` and wait for its acknowledgment;
2. read the current board;
3. treat every update as “stale,” not as a delta;
4. after abrupt close, re-listen and read the current board again; and
5. use no missed-event count, replay offset or stream-local state in settlement.

This closes the initial read/listen race without needing replay. Bursts may still
collapse to one refetch of the latest durable representation; whether and how a host
coalesces invalidations is implementation-specific.

**Inference.** MCP does not define how a commit on World instance A reaches listen
streams currently held by instances B through Z. The SDK can write to the open stream
once the serving process knows about a change; database-to-process invalidation,
multi-instance fan-out, backpressure, quotas and recovery remain server architecture.
That internal fan-out must not become a second source of World truth.

## Official Rust SDK support

**Evidence.** Aicadia pins official `rmcp` `3.1.1` with server and Streamable HTTP
features. The `3.1.1` SDK provides server hooks for accepting a subscription filter
and servicing its stream, a filter-enforcing notification sink, and a client
`listen()` API. The client buffers 64 notifications by default and reports lag; after
disconnect it must call `listen` again because neither HTTP nor stdio subscription
state is resumed. Stateless HTTP uses a fresh handler per request, so shared durable
state cannot live only in a handler.
[Aicadia dependency](../../Cargo.toml#L7-L13),
[`rmcp` 3.1.1 subscriptions](https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.1.1/README.md#subscriptions),
[`rmcp` 3.1.1 release](https://github.com/modelcontextprotocol/rust-sdk/releases/tag/rmcp-v3.1.1)

**Inference.** No replacement MCP library is required for a bounded subscription
experiment. SDK availability does not supply the board model, PostgreSQL fan-out,
authorization, host UX or Agent activation, and it does not prove a million open
streams.

## Authorization and privacy

**Evidence.** MCP authorization is optional. For protected HTTP servers, the current
specification uses OAuth resource-server semantics: a bearer token is sent on every
HTTP request, tokens must be audience-bound to the MCP server, and servers must
validate tokens rather than pass them through. Streamable HTTP separately requires
Origin validation, recommends localhost-only binding for local servers and recommends
authentication for all connections.
[MCP authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization),
[authorization security](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization/security-considerations),
[Streamable HTTP security](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http#security--endpoint)

**Evidence.** A server's resource list may vary by the authorization presented on the
request, but not by hidden connection-local state. Private cache entries may not cross
authorization contexts.
[MCP resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources),
[MCP caching](https://modelcontextprotocol.io/specification/2026-07-28/server/utilities/caching)

**Inference.** A proposal URI can itself disclose an Entity, Place, active dispute or
affected area. Authorization only when opening the stream is insufficient if a
Character's eligibility can later change. At minimum, authoritative refetch and every
write must reauthorize against current durable context. Whether an already-open stream
must stop, suppress later hints or deliberately retain a snapshot-defined audience is
a game/privacy choice, not an MCP answer.

**Candidate implication.** Before hosted subscriptions, define:

- whether eligibility is captured at proposal opening or re-evaluated at every read,
  write and notification;
- whether board discovery and board contents have different visibility;
- maximum concurrent streams and subscribed URIs per authenticated principal;
- notification-rate and slow-consumer limits;
- whether arguments are visible to every eligible User; and
- how player-authored board text remains untrusted data rather than Agent instruction.

A globally guessable Place or Entity URI plus a successful listen acknowledgment must
never be treated as authority to read or act.

## Current Aicadia audit

**Evidence.** Current Aicadia advertises only `tools: {}` and exactly fifteen tools.
It advertises no resources, `resources.subscribe`, list-change capability or logging.
Its server is pinned to protocol `2026-07-28`, rejects `initialize`, uses a
`NeverSessionManager` and a fresh MCP handler backed by a cloned `World` handle.
[MCP handler](../../src/server/mcp.rs#L452-L513),
[server transport](../../src/server/mod.rs#L42-L62)

**Evidence.** The current endpoint is loopback-only, validates one exact local Origin,
disables legacy sessions, SSE keep-alive and SSE retry hints, and requires stateless
protocol metadata. Simple calls prefer JSON responses; the SDK can still use SSE for
a long-lived listen request.
[server transport](../../src/server/mod.rs#L36-L62),
[`rmcp` stateless HTTP](https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.1.1/README.md#stateless-streamable-http)

**Evidence.** `Aicadia-User-Id` is explicitly untrusted development context, not
authentication. Notifications, durable proposals, replay, background target-Agent
activation, clocks, multi-actor commits and server-side intelligence are absent from
the current game contract.
[protocol contract](../game/protocol.md#request-context),
[deferred scope](../game/deferred.md#absent)

**Inference.** A current client cannot meaningfully subscribe to an Aicadia proposal
board because no resource or update capability exists. Adding it is a public game,
World, HTTP/MCP parity, privacy and operational choice—not a transport flag. Current
loopback evidence cannot be extended to hosted multi-User security or massive SSE
fan-out.

## Candidate proposal-board pattern

The following is a research candidate, not a chosen schema or gameplay contract.

### One durable board, hint-only transport

```text
explicit User turn
  -> Agent refetches exact current World and board state
  -> Agent submits bounded candidate package or position
  -> World validates structure/current authority and stores it atomically
  -> MCP transport marks the board resource stale for opted-in connected clients
  -> later explicit User turns refetch and may submit another package or position
  -> deterministic settlement admits one exact final package
  -> World revalidates affected facts and commits state plus Activity atomically
```

**Candidate implication.** A board read could contain a bounded current agenda,
deadline, exact involved subjects and Places, candidate packages, arguments,
positions and settlement state. The corresponding write surface could accept a new
candidate, an amendment, an argument, a position or one settlement package. Those
names and shapes are deliberately undecided.

**Inference.** MCP also supplies no timer or automatic close. A stored deadline can
be compared on a later read and enforced on a later write or explicit settlement
call, but no protocol feature guarantees that a call happens at the deadline.
Real-time closure would require separate accepted game/operation behavior; it still
must not invoke Agents or make their absent votes for them.

The protocol board URI could be Entity-, Place- or deliberation-scoped, but it must
not impose an Entity taxonomy or mandatory Property keys. An Agent proposing “an
explosion affects Places A and B” supplies A and B and a bounded exact change package.
An Agent proposing “fell this tree” supplies the intended sparse state changes and
dependencies that exist in the observed Entity; World does not assume `.form`,
`.exists`, `.shape` or any other key. Other explicitly invoked Agents may refetch the
same board and offer different packages or reasoning.

**Inference.** A new per-deliberation URI cannot announce itself through a
resource-specific subscription to clients that do not know it yet. Core MCP defines
no wildcard resource subscription. A stable already-known Entity or Place board/index
resource can solve that discovery step: its update tells the client to refetch the
current bounded set of boards, after which exact boards may be read. The broader
`resourcesListChanged` signal can announce that the server's entire visible resource
catalog changed, but it is not a locality filter.

**Candidate implication.** For a “nearby Agents” experiment, prefer one stable local
board/index resource already derived from an explicitly observed Entity or current
Place over dynamically subscribing every client to a just-created proposal URI. How
the host changes that subscription when the Character moves, and whether it can do so
without a model turn, must be tested per host; MCP does not maintain Character
presence or interest for Aicadia.

**Inference.** Semantic collective deliberation and database consensus are different.
Agents may compare meanings and invent compromises. World must still use one
previously accepted deterministic settlement rule and one atomic concurrency boundary
to decide which submitted package, if any, becomes current state. MCP subscriptions
solve neither problem; they only reduce discovery latency.

**Candidate implication.** The notification path must remain optional for
correctness. An explicit “check relevant deliberations” turn must be able to obtain
the full current set through bounded MCP reads even when the host never subscribed,
was offline for the entire discussion, or lost every hint.

### Why a custom event notification is a weaker first fit

**Evidence.** The core subscription filter is closed around list changes and resource
updates. MCP permits negotiated extensions, but extension adoption and host behavior
are separate from core compatibility.
[MCP schema](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/2026-07-28/schema.ts#L1261-L1317),
[MCP extensions](https://modelcontextprotocol.io/extensions/overview)

**Inference.** A custom `notifications/aicadia/proposal_opened` message could carry
more domain data, but every host and SDK path would need extension support and it
would tempt clients to treat an ephemeral payload as truth. The standard
resource-update hint already forces the safer refetch boundary.

### Massive concurrency and one hot board

**Inference.** Millions of independent boards are distributable; one board watched
by millions creates millions of live deliveries and potentially millions of explicit
reads or writes. MCP specifies the stream framing, not admission, fairness, batching,
fan-out storage or a hot-board settlement strategy. `rmcp`'s bounded client buffer is
evidence that slow consumers are expected, not evidence that server memory or network
cost is bounded at World scale.

**Candidate implication.** Any experiment must bound at least board size, candidate
count, argument size, positions per eligible actor, subscribed URIs, streams per
principal, open duration, deadline, refetch page size, notification rate, slow-consumer
behavior, write admission and settlement work. A notification may be dropped or
coalesced operationally only if the client subsequently refetches a current durable
representation and no World result depends on delivery count.

One deliberately hot board also needs proof that it has no shared counter row or
unbounded lock queue. A vote-like mechanic cannot silently import a global score or
make every notification a durable per-observer record. Those are Aicadia design
constraints, not properties of MCP.

## Open decisions exposed by the research

1. Which game situations may open a board: explicit common subjects only, or another
   bounded authority?
2. Who creates its initial exact affected scope, and what structural facts can World
   deterministically reject without interpreting meaning?
3. Is participation eligibility snapshotted at opening or current at each operation?
4. Are Agents submitting proposals, approvals, ordered preferences, a final jointly
   signed package, or some other non-score settlement input?
5. Which deterministic rule closes the board, handles a tie or no participation, and
   decides whether a cooldown follows?
6. Is the MCP resource scoped to one board, one Entity, one Place or a private
   relevant-board view, and how is authorization rechecked on a long-lived stream?
7. Which target hosts actually keep `subscriptions/listen` open and how do they expose
   a resource invalidation without invoking an LLM?
8. Which internal multi-instance hint transport reaches all open streams, and what
   bounded behavior occurs when it drops messages or consumers lag?

## What is and is not proved

### Established

- Core MCP `2026-07-28` has an opt-in long-lived subscription with exact resource
  update hints.
- Resource update notifications are naturally followed by authoritative refetch.
- Progress and logging cannot substitute for later independent game changes.
- Streamable HTTP has no protocol session, notification replay or SSE resumption.
- MCP does not prescribe host LLM behavior; a notification is not a model call.
- Sampling is the explicit model-generation mechanism, is request-nested and
  deprecated, and is not available from a subscription stream.
- Aicadia's pinned Rust SDK contains the necessary subscription primitives.
- Current Aicadia exposes tools only and has no notification or resource capability.

### Not established

- that any current Aicadia target host supports or keeps open modern subscriptions;
- that a notification displays anything to a User or can wake a suspended host;
- that any host can or should automatically invoke an Agent from a notification;
- that resource subscription fan-out scales to millions of connected Users;
- that a particular board URI, audience, settlement rule, vote, deadline or cooldown
  is good gameplay;
- that notifications improve semantic consensus quality or reduce token cost;
- that a proposal phase eliminates final-state conflicts or hot-row contention;
- that current loopback User context is safe for hosted collective visibility; or
- that MCP supplies durable delivery, replay, ordering, voting, consensus, privacy or
  authoritative World history.

## Primary sources

- Model Context Protocol,
  [`2026-07-28` subscriptions](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/subscriptions)
- Model Context Protocol,
  [`2026-07-28` Streamable HTTP](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http)
- Model Context Protocol,
  [`2026-07-28` resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources)
- Model Context Protocol,
  [`2026-07-28` caching](https://modelcontextprotocol.io/specification/2026-07-28/server/utilities/caching)
- Model Context Protocol,
  [`2026-07-28` progress](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/progress)
- Model Context Protocol,
  [`2026-07-28` logging](https://modelcontextprotocol.io/specification/2026-07-28/server/utilities/logging)
- Model Context Protocol,
  [`2026-07-28` sampling](https://modelcontextprotocol.io/specification/2026-07-28/client/sampling)
- Model Context Protocol,
  [`2026-07-28` multi round-trip requests](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/mrtr)
- Model Context Protocol,
  [`2026-07-28` authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization)
- Model Context Protocol,
  [`2026-07-28` architecture](https://modelcontextprotocol.io/docs/2026-07-28/learn/architecture)
- Model Context Protocol,
  [normative `2026-07-28` schema](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/2026-07-28/schema.ts)
- Model Context Protocol,
  [official Rust SDK `rmcp` 3.1.1](https://github.com/modelcontextprotocol/rust-sdk/tree/rmcp-v3.1.1)
