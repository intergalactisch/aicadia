---
kind: seam
storage_table: []
---

# World seam

> **Role / side:** World model contract / runtime side.
> **Authority:** the single public game-behavior seam, its adapter boundary and the stand-alone-call rule.
> **Excludes:** subject state, Activity meaning, value validation and delivery status; see the other model contracts and `docs/evidence/`.

The concrete `World` type is the only public game-behavior seam. The fifteen player
capabilities ship together through thin HTTP and MCP adapters. Each explicit call
stands alone: there is no durable game session and no server-side Agent invocation
or inference. Agents may reason and propose, but only World assigns identities,
resolves investigation chance, validates commands and writes durable state.
