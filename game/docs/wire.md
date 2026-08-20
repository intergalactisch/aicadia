# Wire contract

> **Role / side:** Defines shared player-capability response shapes / runtime side.
> **Authority:** the exact compact and complete JSON result shapes shared by World, HTTP and MCP.
> **Excludes:** request identity, transport behavior and errors — defined in [Protocol](protocol.md); model meaning — defined in the [model contracts](README.md#model-contracts).

## Wire shapes

All JSON objects reject unknown fields. Successful operations return the result
directly without a `data` envelope. Timestamps are RFC 3339 strings, ids are UUID
strings, and revisions/cursors are opaque URL-safe strings.

```text
World       { name }
User        { id, created_at }
Entity      { id, name, description, introduced_by_user_id, introduced_at }
Position    { x_cm, y_cm, z_cm, description: string | null,
              position_revision: string }
Place       { entity: Entity, position: Position, is_entry }
Character   { entity: Entity, owner_user_id,
              position: Position | null, current_place: Place | null }
EntitySummary { id, name }
CurrentPlaceEntityOutput { id, name, description, position: Position }
CurrentPlaceOutput { id, name, description, position: Position }
PlaceSummary  { entity: EntitySummary, is_entry }
PlacePositionOutput { id, name, description, is_entry, position: Position }
ConnectionEndpoint { place: PlacePositionOutput }
ConnectionSummary {
  id, source: ConnectionEndpoint, destination: ConnectionEndpoint,
  allows_reverse, name, description, has_course
}
ConnectionPoint { ordinal, x_cm, y_cm, z_cm }
Connection {
  id, source: ConnectionEndpoint, destination: ConnectionEndpoint,
  allows_reverse, name, description, shape_description: string | null,
  course: [ConnectionPoint]
}
PropertyValue { type: "text", text } | { type: "integer", integer }
EntityProperty { entity: EntitySummary, key, value: PropertyValue }
EntityTrait { id, statement }
EntityCurrentAssociation =
  { type: "property", property: { key, value: PropertyValue } } |
  { type: "trait", trait: EntityTrait }
EntityCurrentStatePage {
  association: [EntityCurrentAssociation],
  next: string | null
}
CharacterEntityStatePage {
  character: Character,
  place_revision: string | null,
  current_state: EntityCurrentStatePage
}

ActivityEntityReference {
  entity: EntitySummary,
  role: "subject" | "destination" | "location" | "target"
}
ActivityPositionReference {
  entity: EntitySummary,
  role: "origin" | "result",
  position: Position
}
ActivityConnectionReference {
  id, name, source_place_id, destination_place_id
}
Activity {
  id,
  operation: "create_character" | "create_entity" |
             "create_entry_place" | "enter_world" | "submit_action" |
             "submit_interaction" | "submit_discovery" | "move_character",
  actor_character: EntitySummary | null,
  context_place: PlaceSummary | null,
  involved_entity: [ActivityEntityReference],
  involved_position: [ActivityPositionReference],
  involved_connection: [ActivityConnectionReference],
  property_change: [EntityProperty],
  trait_change: [ActivityTraitChange],
  prose: string | null,
  occurred_at
}
EntityPage   { entity: [EntitySummary], next: string | null }
ActivityPage { activity: [Activity], next: string | null }
PlacePage { place: [PlacePositionOutput], next: string | null }
ConnectionPage {
  place: PlacePositionOutput,
  connection: [ConnectionSummary],
  next: string | null
}
CurrentPlaceEntityPage {
  place: CurrentPlaceOutput,
  place_revision: string,
  entity: [CurrentPlaceEntityOutput],
  next: string | null
}
CurrentPlaceActivityPage {
  place: CurrentPlaceOutput,
  place_revision: string,
  activity: [Activity],
  next: string | null
}
CurrentPlaceEntityStatePage {
  place: CurrentPlaceOutput,
  place_revision: string,
  entity: CurrentPlaceEntityOutput,
  current_state: EntityCurrentStatePage
}
AcceptedAction {
  activity: Activity,
  consequence:
    { type: "introduce_entity", entity: Entity } |
    { type: "change_entity_state",
      property_change: [EntityProperty],
      trait_change: [ActivityTraitChange] },
  place: Place
}
AcceptedInteraction { activity: Activity, place: CurrentPlaceOutput }
InvestigationLimit {
  result_count: 1,
  kind: "entity_at_position" | "connected_place"
}
InvestigationResult {
  attempt_id,
  outcome: "zero" | "positive",
  limit: InvestigationLimit
}
AcceptedDiscovery =
  { type: "entity_at_position", activity, entity, position,
    place: Place | null } |
  { type: "connected_place", activity, origin: Place,
    destination: Place, connection: Connection, character: Character }
AcceptedMovement { activity, character: Character, connection: Connection }
```

Investigation `limit.result_count` is the positive-attempt cap, not a count of finds
created by start. The same immutable limit keeps the zero retry body in that shape.

`CurrentPlaceOutput` is the flat safe current-Place view: id, name, description and
complete Position. Unlike `Place`, it exposes neither complete Entity provenance nor
`is_entry`. Current-Place pages and accepted Interactions use this safe view;
Character, entry and Action results use complete `Place` where their contract
requires it.

An Activity `location` role names the Place where the Activity happened. A `target`
role means only that Interaction behavior was directed toward that Entity; it never
establishes perception, consent, agreement, thought or response. Position `origin`
names exact grounded state and `result` names a version created by the Activity.
Activity Connection references remain compact and never hydrate course points.

Internal User request provenance, accepted request id and fingerprint never appear
in history results. Activity Property and Trait changes are always present, sorted
and empty when none changed. Property changes sort by Entity then key; Trait changes
sort by Entity, stable Trait id and lifecycle tag. Current associations sort
Properties before Traits, then by internal Property-key id or stable Trait id.

Activity, creation and mutation Entity/Place values remain compact acknowledgements.
Only `get_character` and `get_entity_at_current_place` are full player Entity fetches
with current association pages.
