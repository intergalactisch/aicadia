---
status: active
---

# Agent-text lab

> **Role / side:** retained Agent-text experiment track / development side.
> **Authority:** identifies each agent-text lab artifact, its question and where to read it.
> **Excludes:** each experiment's own verdict, status and seams — recorded in its front matter; the texts under test and their methods — defined in `game/mcp/agent/`, `game/docs/`, [public-facing text](../../docs/methodology/public-text.md) and [build-facing text](../../docs/methodology/build-text.md); accepted Agent behavior — defined in `game/docs/agent.md`.

This track holds bounded checks of whether models read Aicadia's texts as intended:
the texts published to playing Agents, and the documents a building Agent reads.
Every experiment is explicit, token-bounded and limited to the exact models,
prompts, questions or tasks it ran; none proves universal compliance.

The methods that call for these probes are
[`public-text.md`](../../docs/methodology/public-text.md) and
[`build-text.md`](../../docs/methodology/build-text.md).

## Experiments

- [01 — Comprehension probe](01-comprehension-probe/README.md) — Do two model classes answer a fixed rule-question set at least as well from the rewritten Agent texts as from the previous ones?
- [02 — Builder wayfinding probe](02-builder-wayfinding-probe/README.md) — Does a small model name the owning file for six change tasks at least as often from the wayfinding-header texts as from the previous ones?
