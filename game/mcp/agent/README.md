# Agent contract sources

> **Role / side:** published Agent-text source index / runtime side.
> **Authority:** identifies the source files whose exact bytes are published to Agents.
> **Excludes:** independent game rules, delivery status and build planning.

These source files are published verbatim to every Agent. The files under
`instruction/` are the sections of the global play contract, assembled in order by
`game/src/agent_contract.rs`; each file under `tool/` is one tool description.

Formatting is load-bearing and protected by boundary-anchor tests and the catalog
fixture. Any wording change is a change to the public Agent contract: follow the
public-text change procedure (inventory, parity, one fixture regeneration) rather
than editing incidentally.
