# PostgreSQL and MCP interest-strategy comparison

> **Role / side:** retained real-seam multiplayer experiment / development side.
> **Authority:** records this fixture, its bounded observations, verdict and status.
> **Excludes:** current game behavior, production schema or transport, host-product support, authority design and scale proof.

Status: `kept`

Verdict: `inconclusive`

## Retained final experiment choice

Recorded: 2026-08-17.

The experiment closes with four explicit choices:

1. retain structural World/area/Place plus exact-Entity interest as the only
   candidate that advances from this comparison, because it alone had zero required
   coverage misses and zero irrelevant wakes in the fixed direct fixture;
2. retain sixty-four resource-lock stripes as the next lab router candidate when hot
   and quiet resources must share one gateway instance, because it reduced fixed
   hot-lock collision exposure to one in sixty-four with equal pair capacity and no
   material throughput difference;
3. retain coalescible dirty hints plus authoritative reads as the correctness shape,
   and handle a forced-fatal listener by visibly ending the stale stream followed by
   replacement `listen` and then a bounded baseline read; and
4. retain every other strategy and lock variant only as comparative evidence, not as
   an active candidate.

These are final choices for this retained experiment, not accepted production
behavior. Q8 remains open because the striped router is not integrated into the
existing Gateway and hosted clients, public networking, authorization and
million-subscriber capacity remain unproved. The next multiplayer lab question is
simultaneous requests against one Entity.

## Pending decision

Multiplayer Q8 deliberately remains unanswered. Aicadia needs one lightweight live
interest system that catches relevant unknown local change, exact Entity focus,
multi-Place and broader structural effects without turning every local mutation into
a global wake-up.

This experiment compares five forms through one shared implementation and fixture:

1. a globally invalidated World board;
2. the current Place board;
3. exact selected Entities only;
4. current Place plus exact selected Entities; and
5. a structural World/area/Place chain plus exact selected Entities.

World scope is only an experimental change scope. The fixture injects it directly;
it contains no role, right, admin identity or player permission. A future admin is a
possible caller, not part of this experiment.

## Falsifiable question

Which strategy preserves the required live game coverage with the smallest bounded
database, gateway, MCP notification and refetch cost across identical local,
cross-Place, regional, World-scope, movement, hotspot, quiet-subject and reconnect
scenarios?

A fast variant fails if it misses required coverage. A complete variant fails the
boundedness gate if it needs recipient rows, an unbounded pending queue, one database
connection per subscription, every local change on a global resource, or automatic
Agent invocation.

## Evidence boundary

The executed real tier uses a scratch PostgreSQL database, SQLx transactions,
post-commit `LISTEN`/`NOTIFY`, two gateway listeners, loopback MCP `2026-07-28`
Streamable HTTP, `subscriptions/listen`, resource-update notifications and
authoritative resource reads. It remains an experimental schema and server.

Claude Code, ChatGPT, Codex, public networking, production Aicadia World and auth,
LLMs, one million sockets and hosted capacity are absent. A separate synthetic tier
streams one million interest records through the same strategy key-selection code,
folds the full population into cohort counts and retains 8,192 executable recipients;
it does not use the real Gateway router and is not a million-User or
million-connection claim.

The lab host identity is a caller-supplied HTTP header. Subscription admission checks
only the owned exact URI shape, not resource existence or host authority; exact
acknowledgment therefore proves filter round-trip behavior, not authorization.

## Current bounds

- Model, Agent and remote-service calls: zero.
- Direct tier: one writer, two gateways, 32 MCP subscribers and 100 committed
  changes per strategy, sequentially and bounded to 30 seconds per strategy.
- Synthetic tier: 1,000,000 interest records and 10,000 fixed skewed changes per
  strategy.
- Follow-up router tier: 4,096 hot recipients, 64 quiet recipients, 4 hot and 4
  quiet producers, 7 fixed repeats and equal 4,160-key capacity for every variant.
- Follow-up failure tier: one disposable World, one failed and one replacement
  PgListener/Gateway/MCP chain, one offline accepted change and zero model calls.
- One accepted change stores current state plus exactly one Activity; subscription
  state is transient and never stored per recipient.
- Repeated dirtiness may coalesce to one pending `(host, resource)` key; refetch
  returns current state plus bounded recent Activities.

## Direct-tier result — 2026-08-16

The direct tier passed over the intended real chain:

```text
scratch PostgreSQL -> World transaction + Activity -> pg_notify
-> two independent PgListeners -> two loopback rmcp Streamable HTTP servers
-> 32 subscriptions/listen streams -> notifications/resources/updated
-> authoritative resources/read
```

Both gateway instances were actually exercised: each held 16 active MCP
subscriptions and each observed the exact World notification count for every
strategy. This is replicated PostgreSQL notification fan-out, not partitioned
gateway work; database/listener cost as gateway count grows remains unmeasured. Each
strategy committed the same 100-step `ScenarioProgram`. Initial connection, movement
and reconnection all opened `subscriptions/listen` before an authoritative read-all
baseline. Reconnection counted as recovery only when a returned document contained
the Activity committed while that host was offline.

The isolated command completed in 27.30 seconds total; every strategy remained
below its individual 30-second bound:

```shell
DATABASE_URL=postgres://localhost/postgres cargo test \
  --manifest-path lab/multiplayer/05-postgres-mcp-interest-strategies/Cargo.toml \
  --test real_strategy_matrix -- --test-threads=1 --nocapture
```

| Strategy | NOTIFY / gateway | Unroutable commits | Expected = observed resource updates | Live misses | Recovery misses | Irrelevant wakes | Refetches | MCP JSON bytes | SQL statements | Mutation p95 | Commit→all expected MCP updates p95 | Elapsed |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Global firehose | 100 | 0 | 3,186 = 3,186 | 0 | 0 | 2,242 | 3,220 | 34,255,286 | 3,817 | 3,386 µs | 3,539 µs | 11,682 ms |
| Place | 98 | 2 | 710 = 710 | 247 | 1 | 0 | 744 | 8,402,507 | 2,827 | 2,784 µs | 1,159 µs | 3,563 ms |
| Exact only | 97 | 3 | 388 = 388 | 556 | 0 | 0 | 396 | 3,942,403 | 1,386 | 2,656 µs | 923 µs | 1,761 ms |
| Place + exact | 98 | 2 | 1,098 = 1,098 | 48 | 0 | 0 | 1,140 | 12,344,910 | 3,619 | 3,171 µs | 1,755 µs | 4,975 ms |
| Structural | 100 | 0 | 1,146 = 1,146 | 0 | 0 | 0 | 1,256 | 12,450,252 | 3,969 | 3,168 µs | 1,833 µs | 5,145 ms |

Each MCP latency sample runs from commit return until the runner has sequentially
drained every expected rmcp update for that commit; it is not per-notification
latency. Byte counts cover returned JSON document text, not HTTP framing. Thirteen
coalesced dirty checks for every exact-bearing strategy are reported separately.
All strategies ended with zero pending keys, listener errors, malformed payloads,
dropped hints, overload terminations and closed-sink terminations. No model or Agent
was invoked. The successful cleanup audit found no registered database for any of
this lab's eight SQLx test paths.

### Direct-tier verdict

Structural interest is the only candidate supported by this fixture: it preserved
all required local, unknown-Entity, multi-Place, area, World, movement and reconnect
coverage without irrelevant host wake-ups. Global firehose also covered every
required host but caused 2,242 irrelevant wake-ups and about 2.75 times the refetch
bytes of structural interest. Place, exact-only and Place-plus-exact are refuted as
complete strategies by their coverage misses. Empty route-key sets intentionally
produced no dummy notification; their 2/3/2 unroutable commits remain visible as
coverage failures.

The experiment's overall verdict remains `inconclusive`: this direct result does
not prove public host-product support, remote networking, authentication, production
World integration, million-connection capacity or the separate synthetic scale
tier. The fixed semantic audience and coverage oracle are simulated test inputs;
the database, World writes and reads, both PgListeners, both HTTP servers, all 32
rmcp clients, acknowledgements, updates and reads are real.

Overload behavior has one additional focused real-rmcp check. Hitting a sink or
pending-key bound unregisters the affected host and gracefully ends its stream; a
new listen followed by read-all recovered the changed documents. That focused check
uses the in-memory authoritative reader, so PostgreSQL overload recovery itself is
not claimed. The direct-tier Gateway still uses one instance-wide mutex; the focused
follow-up below tests a separate striped candidate rather than silently changing or
crediting that Gateway.

## Follow-up — hot isolation and fatal listener recovery

One retained Rust router compared one, eight and sixty-four resource-lock stripes.
Every variant registered the same 4,160 `(resource, recipient)` pairs, reserved the
same 4,160 pending keys and ran seven repeats with 4,096 recipients on one hot
resource, 64 quiet resources, four concurrent hot producers and four concurrent
quiet producers. Each variant produced exactly 7,168 hot routes, 29,360,128 hot
matches, 114,688 quiet routes, 29,120 newly dirty keys, 29,445,696 coalesced matches,
zero drops and zero pending teardown state.

| Stripes | Quiet routes sharing hot lock | Quiet latency p50 / p95 / p99 / max | Quiet lock wait p50 / p95 / p99 / max | Hot routes/s |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 114,688 / 114,688 | 0.167 / 9.541 / 25.708 / 61,072.667 µs | 0.042 / 9.375 / 25.542 / 61,071.417 µs | 16,865 |
| 8 | 14,336 / 114,688 | 0.125 / 0.250 / 0.375 / 60,372.542 µs | 0.042 / 0.125 / 0.250 / 60,370.000 µs | 16,877 |
| 64 | 1,792 / 114,688 | 0.166 / 0.292 / 0.375 / 60,176.000 µs | 0.042 / 0.125 / 0.208 / 60,174.417 µs | 16,955 |

These values are from the final isolated run after correcting aggregate throughput
to use one shared start per repeat. Independent reruns produced materially different
absolute scheduler-driven maxima and throughput, so all timing remains descriptive. The
selection evidence is structural: with equal total capacity and essentially equal
hot throughput, sixty-four stripes reduced deterministic hot-lock collision exposure
from every quiet route to one in sixty-four. It is therefore the strongest candidate
inside this fixture, not a production constant or hosted-capacity result. The modulo
fixture hash, per-stripe capacity allocation, fairness, backpressure and maximum-hot
quiet latency on deployed hardware remain open.

The 4,160 capacity here counts exact resource-recipient pairs. It does not model the
existing Gateway's separate host-admission limit or its per-host resource limit, so
the candidate is not a drop-in Gateway or admission design. Precomputing each
stripe's capacity from the fixed workload is valid only for this comparison.

The candidate also passed a focused real-rmcp smoke. A genuine
`subscriptions/listen` registered its exact pair in the sixty-four-stripe router;
that same router selected the newly dirty recipient after releasing its stripe lock,
the client received `notifications/resources/updated`, and an authoritative resource
read cleared only that pending pair. A repeated route coalesced before the read and a
later route emitted again after it. This uses the real rmcp protocol path with an
in-memory authoritative document; it does not use PostgreSQL, the experiment's
existing Gateway, a public host product or a scale population.

A separate real SQLx scenario closed the planned forced-fatal listener-pool case.
Closing the dedicated one-connection pool made the actual `PgListener` fail,
terminated the old MCP stream and its transient subscription state, and prevented
the failed Gateway from remaining healthy-looking. A World change then committed
while no listener was present. A replacement Gateway opened a new listen first and
then read an authoritative baseline containing both revision 1 and the exact missed
Activity. No notification replay occurred or was required. SQLx transient reconnect,
network partitions and production listener behavior remain unproved.

### Follow-up verdict

`supported` for this bounded follow-up in three separate claims: the fixed striped
router isolates disjoint resource locks; that same router preserves the exercised
one-client rmcp listen/update/coalesce/read success path; and fatal PostgreSQL
listener loss converges through visible stream termination plus
listen-then-baseline recovery. The rmcp smoke does not cover failed-read restoration,
sink overflow or the hot/quiet workload itself. Experiment 05 as a whole remains
`inconclusive` and does not select Aicadia's production interest contract: the
striped router is not integrated into the existing Gateway, host products and public
networking are absent, and millions of live subscribers, cross-gateway broadcast
cost, authentication, concurrent World writes and production failure behavior remain
unproved.

## Synthetic-tier result — 2026-08-16

The release run processed 1,000,000 generated interest records and 10,000 fixed
skewed changes for every strategy. Full-population raw key matches are exact cohort
sums before per-host deduplication. Coverage, coalescing and refetch behavior execute
only for the retained 8,192-host sample; full-population dirty-key and refetch values
are projections from that sample. Within-Area and cross-Area movement are measured
separately instead of assuming every move crosses an Area boundary.

```shell
cargo test --release \
  --manifest-path lab/multiplayer/05-postgres-mcp-interest-strategies/Cargo.toml \
  --test scale_matrix -- --ignored --nocapture --test-threads=1
```

| Strategy | Indexed key increments / unique cohorts | Exact full raw matches | Sample coverage misses | Sample irrelevant wakes | Sample within / cross-Area churn | Route p95 | Ingest / routing |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Global firehose | 1,000,000 / 1 | 10,000,000,000 | 0 | 61,381,111 | 0 / 0 | 353 µs | 21,166 / 2,819,533 µs |
| Place | 1,000,000 / 16,384 | 1,400,111,886 | 9,064,937 | 0 | 16,384 / 16,384 | 251 µs | 20,923 / 1,748,151 µs |
| Exact only | 1,000,000 / 243,852 | 1,350,012,145 | 9,473,300 | 0 | 0 / 0 | 164 µs | 40,357 / 853,743 µs |
| Place + exact | 2,000,000 / 260,236 | 2,750,124,031 | 1,689,356 | 0 | 16,384 / 16,384 | 320 µs | 64,717 / 2,089,924 µs |
| Structural | 4,000,000 / 260,365 | 2,956,322,744 | 0 | 0 | 16,384 / 32,768 | 328 µs | 93,932 / 2,267,938 µs |

Each route-latency sample includes an O(8,192) semantic relevance-oracle scan. These
times are therefore not pure Gateway routing measurements and do not compare
PostgreSQL, MCP, network or hosted throughput. The five-strategy release command
completed in 10.04 seconds on this local run; that wall time is descriptive only.
The synthetic tier does not establish production memory, capacity, quiet-subject
latency, one million live subscriptions or one million supported Users.

### Synthetic-tier verdict

The synthetic topology agrees with the fixed direct coverage ordering: global and
structural have no sampled semantic misses, the global form wakes every sampled host,
and structural pays additional scope-chain keys and cross-Area churn. This supports
only the comparison's topology arithmetic. It does not close the experiment's
overall `inconclusive` verdict or select Q8's product behavior.

## Bounded database and failure evidence

The first implementation returned at most 32 Activities but could still scan tens
of thousands of newer unrelated rows for a dormant subject. The retained fixture
replaced those array-membership history reads with at most 49 bounded,
non-recipient `activity_resource` rows per Activity and an ordered
`(resource_uri, recorded_at DESC, activity_id DESC)` index. A disposable 100,000-row
adversarial `EXPLAIN (ANALYZE, BUFFERS)` fixture then observed:

| Read | Rows from ordered path | Buffers | Local elapsed |
| --- | ---: | ---: | ---: |
| Global | 32 | 5 | 0.028 ms |
| World | 10 | 53 | 0.098 ms |
| Area | 32 | 163 | 0.224 ms |
| Formerly hot, now quiet Place | 32 | 133 | 0.184 ms |
| Formerly hot, now quiet Entity | 32 | 133 | 0.166 ms |

The quiet Place and Entity each had 50,000 newer unrelated Activities; none were
filtered. These timings are local plan evidence, not hosted throughput. Operation and
resource names are capped at 128 bytes, stored state at 4,096 bytes and a complete
resource response at 2 MiB. Child discovery returns at most 128 rows and exposes a
truncation flag.

Post-commit notification has a 100 ms lab timeout. Exhausting that budget loses only
the hint and never rolls back already committed state or Activity. One `PgListener`
holds one pool connection per gateway instance, while all 16 MCP subscribers behind
that instance share it. The direct tier remains one sequential writer and proves no
concurrent write capacity.

## Validation and cleanup

```shell
cargo fmt --manifest-path \
  lab/multiplayer/05-postgres-mcp-interest-strategies/Cargo.toml -- --check
cargo check --tests --locked --manifest-path \
  lab/multiplayer/05-postgres-mcp-interest-strategies/Cargo.toml
cargo clippy --tests --locked --manifest-path \
  lab/multiplayer/05-postgres-mcp-interest-strategies/Cargo.toml -- -D warnings
cargo test --release --locked --manifest-path \
  lab/multiplayer/05-postgres-mcp-interest-strategies/Cargo.toml \
  fanout::tests::release_hot_fanout_matrix -- --ignored --nocapture
DATABASE_URL=postgres://localhost/postgres cargo test --locked \
  --manifest-path lab/multiplayer/05-postgres-mcp-interest-strategies/Cargo.toml \
  --test listener_failure_recovery --test striped_mcp_smoke -- --test-threads=1
DATABASE_URL=postgres://localhost/postgres cargo test --locked \
  --manifest-path lab/multiplayer/05-postgres-mcp-interest-strategies/Cargo.toml \
  -- --test-threads=1
DATABASE_URL=postgres://localhost/postgres cargo test --locked \
  --manifest-path lab/multiplayer/05-postgres-mcp-interest-strategies/Cargo.toml \
  --test cleanup_audit -- --ignored --test-threads=1
```

The integrated suite, Clippy gate, release scale matrix and separate cleanup audit
passed. No database registered to this experiment's eight SQLx test paths was
retained; unrelated pre-existing SQLx test registrations are outside this audit.
