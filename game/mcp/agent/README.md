# Agent contract sources

> **Role / side:** published Agent-text source index / runtime side.
> **Authority:** identifies the source files whose exact bytes are published to Agents.
> **Excludes:** independent game rules, delivery status and build planning.

These source files are published verbatim to every Agent. `instruction.md` is the
global play contract; each file under `tool/` is one tool description.

Formatting is load-bearing and protected by pinned-phrase tests and the catalog
fixture. “Fixing the Markdown” changes the public Agent contract and therefore
requires the accepted Agent-text rewrite plan rather than an incidental edit.
