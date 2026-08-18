use chrono::TimeZone;

use super::*;

#[test]
fn cursor_round_trips_and_rejects_invalid_input() {
    let cursor = EntityCursor {
        introduced_at: Utc
            .with_ymd_and_hms(2026, 8, 7, 12, 30, 0)
            .single()
            .expect("timestamp should be valid"),
        entity_id: EntityId(Uuid::new_v4()),
    };

    let activity_cursor = ActivityCursor {
        occurred_at: cursor.introduced_at,
        activity_id: ActivityId(Uuid::new_v4()),
    };
    assert_eq!(
        decode_activity_cursor(&encode_activity_cursor(activity_cursor)),
        Ok(activity_cursor)
    );
    assert_eq!(
        decode_place_entity_cursor(&encode_place_entity_cursor(cursor)),
        Ok(cursor)
    );
    assert_eq!(
        decode_place_activity_cursor(&encode_place_activity_cursor(activity_cursor)),
        Ok(activity_cursor)
    );
    let property_cursor = EntityCurrentStateCursor::from_property(
        cursor.entity_id,
        Some(PlaceRevision::from_parts(
            cursor.entity_id,
            cursor.introduced_at,
            activity_cursor.activity_id,
        )),
        17,
    );
    assert_eq!(
        decode_character_state_cursor(&encode_character_state_cursor(property_cursor)),
        Ok(property_cursor)
    );
    assert_eq!(
        decode_current_place_entity_state_cursor(&encode_current_place_entity_state_cursor(
            property_cursor,
        )),
        Ok(property_cursor)
    );
    assert_eq!(
        decode_current_place_entity_state_cursor(&encode_character_state_cursor(property_cursor,)),
        Err(ErrorOutput::invalid_cursor())
    );
    let trait_cursor =
        EntityCurrentStateCursor::from_trait(cursor.entity_id, None, EntityTraitId(Uuid::new_v4()));
    assert_eq!(
        decode_character_state_cursor(&encode_character_state_cursor(trait_cursor)),
        Ok(trait_cursor)
    );

    for encoded in [
        encode_place_entity_cursor(cursor),
        encode_place_activity_cursor(activity_cursor),
    ] {
        assert_eq!(
            decode_activity_cursor(&encoded),
            Err(ErrorOutput::invalid_cursor())
        );
    }
    for encoded in [
        encode_activity_cursor(activity_cursor),
        encode_place_activity_cursor(activity_cursor),
    ] {
        assert_eq!(
            decode_place_entity_cursor(&encoded),
            Err(ErrorOutput::invalid_cursor())
        );
    }
    for encoded in [
        encode_activity_cursor(activity_cursor),
        encode_place_entity_cursor(cursor),
    ] {
        assert_eq!(
            decode_place_activity_cursor(&encoded),
            Err(ErrorOutput::invalid_cursor())
        );
    }

    let revision = PlaceRevision::from_parts(
        cursor.entity_id,
        cursor.introduced_at,
        activity_cursor.activity_id,
    );
    assert_eq!(
        decode_place_revision(&encode_place_revision(revision)),
        Ok(revision)
    );
    assert_eq!(
        decode_place_revision(&encode_activity_cursor(activity_cursor)),
        Err(ErrorOutput::invalid_place_revision())
    );
}

#[test]
fn error_codes_have_one_compiler_checked_wire_spelling() {
    for code in [
        ErrorCode::UserContextRequired,
        ErrorCode::InvalidRequest,
        ErrorCode::InvalidEntity,
        ErrorCode::InvalidCharacter,
        ErrorCode::InvalidPlace,
        ErrorCode::InvalidAction,
        ErrorCode::InvalidInteraction,
        ErrorCode::InvalidDiscovery,
        ErrorCode::InvalidProperty,
        ErrorCode::InvalidTrait,
        ErrorCode::InvalidEntityLimit,
        ErrorCode::InvalidActivityLimit,
        ErrorCode::UserNotFound,
        ErrorCode::CharacterNotFound,
        ErrorCode::CharacterAlreadyExists,
        ErrorCode::CharacterAlreadyEntered,
        ErrorCode::CharacterNotEntered,
        ErrorCode::EntryPlaceAlreadyExists,
        ErrorCode::EntryPlaceNotFound,
        ErrorCode::ActionRequestConflict,
        ErrorCode::InteractionRequestConflict,
        ErrorCode::DiscoveryRequestConflict,
        ErrorCode::DiscoveryAttemptUnavailable,
        ErrorCode::InteractionTargetUnavailable,
        ErrorCode::PropertyEntityUnavailable,
        ErrorCode::EntityAtCurrentPlaceUnavailable,
        ErrorCode::TraitUnavailable,
        ErrorCode::PropertyKeyConflict,
        ErrorCode::PlaceRevisionConflict,
        ErrorCode::InvestigationNotAdmitted,
        ErrorCode::Unavailable,
    ] {
        // Exhaustive on purpose: a new ErrorCode without a published spelling
        // stops compiling here instead of shipping as an unproven public code.
        let spelling = match code {
            ErrorCode::UserContextRequired => "user_context_required",
            ErrorCode::InvalidRequest => "invalid_request",
            ErrorCode::InvalidEntity => "invalid_entity",
            ErrorCode::InvalidCharacter => "invalid_character",
            ErrorCode::InvalidPlace => "invalid_place",
            ErrorCode::InvalidAction => "invalid_action",
            ErrorCode::InvalidInteraction => "invalid_interaction",
            ErrorCode::InvalidDiscovery => "invalid_discovery",
            ErrorCode::InvalidProperty => "invalid_property",
            ErrorCode::InvalidTrait => "invalid_trait",
            ErrorCode::InvalidEntityLimit => "invalid_entity_limit",
            ErrorCode::InvalidActivityLimit => "invalid_activity_limit",
            ErrorCode::UserNotFound => "user_not_found",
            ErrorCode::CharacterNotFound => "character_not_found",
            ErrorCode::CharacterAlreadyExists => "character_already_exists",
            ErrorCode::CharacterAlreadyEntered => "character_already_entered",
            ErrorCode::CharacterNotEntered => "character_not_entered",
            ErrorCode::EntryPlaceAlreadyExists => "entry_place_already_exists",
            ErrorCode::EntryPlaceNotFound => "entry_place_not_found",
            ErrorCode::ActionRequestConflict => "action_request_conflict",
            ErrorCode::InteractionRequestConflict => "interaction_request_conflict",
            ErrorCode::DiscoveryRequestConflict => "discovery_request_conflict",
            ErrorCode::DiscoveryAttemptUnavailable => "discovery_attempt_unavailable",
            ErrorCode::InteractionTargetUnavailable => "interaction_target_unavailable",
            ErrorCode::PropertyEntityUnavailable => "property_entity_unavailable",
            ErrorCode::EntityAtCurrentPlaceUnavailable => "entity_at_current_place_unavailable",
            ErrorCode::TraitUnavailable => "trait_unavailable",
            ErrorCode::PropertyKeyConflict => "property_key_conflict",
            ErrorCode::PlaceRevisionConflict => "place_revision_conflict",
            ErrorCode::InvestigationNotAdmitted => "investigation_not_admitted",
            ErrorCode::Unavailable => "unavailable",
        };
        assert_eq!(
            serde_json::to_value(code).expect("error code should serialize"),
            serde_json::json!(spelling)
        );
    }
}

#[test]
fn action_wire_accepts_only_the_current_combined_state_variant() {
    let request_id = Uuid::new_v4();
    let current = serde_json::json!({
        "request_id": request_id,
        "expected_place_revision": "opaque-unparsed-here",
        "prose": "One state package changes.",
        "consequence": {
            "type": "change_entity_state",
            "property_change": [],
            "trait_change": []
        }
    });
    assert!(serde_json::from_value::<SubmitActionInput>(current).is_ok());

    for old_type in ["change_entity_property", "change_entity_trait"] {
        let old = serde_json::json!({
            "request_id": request_id,
            "expected_place_revision": "opaque-unparsed-here",
            "prose": "An obsolete state package is rejected.",
            "consequence": {
                "type": old_type,
                "property_change": [],
                "trait_change": []
            }
        });
        assert!(
            serde_json::from_value::<SubmitActionInput>(old).is_err(),
            "the superseded {old_type} public input must stay absent"
        );
    }
}
