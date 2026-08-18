//! Server-rendered Live-section route evidence over one disposable World.

use aicadia::{
    AcceptedActionConsequence, ActionConsequence, ChangeEntityState, CreateCharacter,
    CreateEntryPlace, EntityCurrentAssociation, EntityId, EntityPropertyChangeInput,
    EntityTraitChangeInput, EntityTraitId, GetEntityAtCurrentPlace, IntroduceEntity,
    ListEntityAtCurrentPlace, PlaceRevision, PropertyInput, PropertyValue, SubmitAction,
    TraitInput, World,
};
use aicadia_studio as studio;
use reqwest::{Client, StatusCode};
use sqlx::PgPool;
use tokio::{net::TcpListener, task::JoinHandle};
use uuid::Uuid;

struct StudioServer {
    base_url: String,
    client: Client,
    task: JoinHandle<()>,
}

impl StudioServer {
    async fn start(pool: PgPool) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("loopback listener should bind");
        let address = listener.local_addr().expect("listener has an address");
        let router = studio::app(World::new(pool.clone()), pool);
        let task = tokio::spawn(async move {
            axum::serve(listener, router)
                .await
                .expect("Studio should serve");
        });
        Self {
            base_url: format!("http://{address}"),
            client: Client::new(),
            task,
        }
    }

    async fn html(&self, path: &str) -> String {
        let response = self
            .client
            .get(format!("{}{path}", self.base_url))
            .send()
            .await
            .unwrap_or_else(|error| panic!("{path} should send: {error}"));
        assert_eq!(response.status(), StatusCode::OK, "{path} should render");
        response.text().await.expect("page should be text")
    }
}

impl Drop for StudioServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

struct WalkSeed {
    user_id: Uuid,
    character_id: Uuid,
    place_id: Uuid,
    entity_id: Uuid,
    trait_id: Uuid,
    property_key_id: i64,
    activity_id: Uuid,
}

async fn current_revision(world: &World, user_id: Uuid) -> PlaceRevision {
    world
        .list_entity_at_current_place(
            aicadia::UserId(user_id),
            ListEntityAtCurrentPlace::default(),
        )
        .await
        .expect("the Character can read its Place")
        .place_revision
}

async fn trait_id_of(world: &World, user_id: Uuid, entity_id: Uuid) -> EntityTraitId {
    world
        .get_entity_at_current_place(
            aicadia::UserId(user_id),
            GetEntityAtCurrentPlace {
                entity_id: EntityId(entity_id),
                cursor: None,
                limit: 100,
            },
        )
        .await
        .expect("the Entity is readable")
        .current_state
        .association
        .into_iter()
        .find_map(|association| match association {
            EntityCurrentAssociation::Trait(value) => Some(value.id),
            EntityCurrentAssociation::Property { .. } => None,
        })
        .expect("the Entity has one Trait")
}

async fn seed(pool: &PgPool) -> WalkSeed {
    let world = World::new(pool.clone());
    let user = world.create_user().await.expect("a User is created");
    let character = world
        .create_character(
            user.id,
            CreateCharacter {
                name: "Mara Venn".to_owned(),
                description: "A careful surveyor at the edge of the known World.".to_owned(),
                property: vec![PropertyInput {
                    key: "role".to_owned(),
                    value: PropertyValue::Text("surveyor".to_owned()),
                }],
                r#trait: vec![TraitInput {
                    statement: "Keeps a field journal of every measurement.".to_owned(),
                }],
            },
        )
        .await
        .expect("the User creates a Character");
    let place = world
        .create_entry_place(
            user.id,
            CreateEntryPlace {
                name: "North Gate".to_owned(),
                description: "The established entry into the shared World.".to_owned(),
                property: vec![],
                r#trait: vec![],
            },
        )
        .await
        .expect("the World gains an entry Place");
    world
        .enter_world(user.id)
        .await
        .expect("the Character enters the World");

    let introduced = world
        .submit_action(
            user.id,
            SubmitAction {
                request_id: Uuid::new_v4(),
                expected_place_revision: current_revision(&world, user.id.0).await,
                prose: "Mara sets a marker stone beside the gate.".to_owned(),
                consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                    name: "Marker stone".to_owned(),
                    description: "A weathered survey marker leaning north.".to_owned(),
                    property: vec![PropertyInput {
                        key: "height_cm".to_owned(),
                        value: PropertyValue::Integer(63),
                    }],
                    r#trait: vec![TraitInput {
                        statement: "Stands one pace from the gate.".to_owned(),
                    }],
                }),
            },
        )
        .await
        .expect("the Character introduces an Entity");
    let entity_id = match introduced.consequence {
        AcceptedActionConsequence::IntroduceEntity(entity) => entity.id.0,
        AcceptedActionConsequence::ChangeEntityState { .. } => {
            panic!("the introduce action returns its Entity")
        }
    };
    let trait_id = trait_id_of(&world, user.id.0, entity_id).await;
    let changed = world
        .submit_action(
            user.id,
            SubmitAction {
                request_id: Uuid::new_v4(),
                expected_place_revision: current_revision(&world, user.id.0).await,
                prose: "Mara re-measures the marker and corrects her field note.".to_owned(),
                consequence: ActionConsequence::ChangeEntityState(ChangeEntityState {
                    property_change: vec![EntityPropertyChangeInput {
                        entity_id: EntityId(entity_id),
                        key: "height_cm".to_owned(),
                        value: PropertyValue::Integer(64),
                    }],
                    trait_change: vec![EntityTraitChangeInput::Develop {
                        trait_id,
                        statement: "Stands one pace from the gate, leaning north.".to_owned(),
                    }],
                }),
            },
        )
        .await
        .expect("the state change is accepted");
    let property_key_id =
        sqlx::query_scalar::<_, i64>("SELECT id FROM property_key WHERE key = 'height_cm'")
            .fetch_one(pool)
            .await
            .expect("the Property key is stored");

    WalkSeed {
        user_id: user.id.0,
        character_id: character.entity.id.0,
        place_id: place.entity.id.0,
        entity_id,
        trait_id: trait_id.0,
        property_key_id,
        activity_id: changed.activity.id.0,
    }
}

#[sqlx::test(migrations = "../game/migration")]
async fn the_live_world_is_one_cross_linked_walk_with_stable_record_urls(pool: PgPool) {
    let seed = seed(&pool).await;
    let server = StudioServer::start(pool).await;

    let world = server.html("/live").await;
    assert!(world.contains("Enter the World"));
    assert!(world.contains(&format!("href=\"/live/place/{}\"", seed.place_id)));
    assert!(world.contains("local-development sort"));

    let place = server.html(&format!("/live/place/{}", seed.place_id)).await;
    assert!(place.contains("Characters here"));
    assert!(place.contains(&format!("href=\"/live/character/{}\"", seed.character_id)));
    assert!(place.contains("href=\"/live/place\" aria-current=\"page\""));

    let character = server
        .html(&format!("/live/character/{}", seed.character_id))
        .await;
    assert!(character.contains("Character chronicle"));
    assert!(character.contains(&format!("href=\"/live/activity/{}\"", seed.activity_id)));
    assert!(character.contains("href=\"/live/character\" aria-current=\"page\""));

    let activity = server
        .html(&format!("/live/activity/{}", seed.activity_id))
        .await;
    assert!(activity.contains("Property changes"));
    assert!(activity.contains(&format!("href=\"/live/entity/{}\"", seed.entity_id)));
    assert!(activity.contains("href=\"/live/activity\" aria-current=\"page\""));

    let entity = server
        .html(&format!("/live/entity/{}", seed.entity_id))
        .await;
    assert!(entity.contains("Current Properties"));
    assert!(entity.contains("href=\"/live/entity\" aria-current=\"page\""));
    assert!(entity.contains(&format!("href=\"/live/trait/{}\"", seed.trait_id)));
    assert!(entity.contains(&format!(
        "href=\"/live/entity/{}/property/{}\"",
        seed.entity_id, seed.property_key_id
    )));
    assert!(entity.contains("/live/storage/entity#row-"));

    let r#trait = server.html(&format!("/live/trait/{}", seed.trait_id)).await;
    assert!(r#trait.contains("Stands one pace from the gate, leaning north."));
    assert!(r#trait.contains(&format!("href=\"/live/entity/{}\"", seed.entity_id)));

    let history = server
        .html(&format!(
            "/live/entity/{}/property/{}",
            seed.entity_id, seed.property_key_id
        ))
        .await;
    assert!(history.contains("Versions"));
    assert!(history.contains("href=\"/live/property-key/height_cm\""));

    let key = server.html("/live/property-key/height_cm").await;
    assert!(key.contains("first accepted use"));
    assert!(!key.contains("Current holders"));
    assert!(key.contains("href=\"/live/property-key\" aria-current=\"page\""));

    let table = server.html("/live/storage/entity").await;
    assert!(table.contains("Rows"));
    assert!(table.contains(&seed.entity_id.to_string()));
    assert!(table.contains("id=\"row-"));
    assert!(table.contains("href=\"/live/storage\""));
    assert!(table.contains("href=\"/live/storage/entity\" aria-current=\"page\""));

    let reloaded = server
        .html(&format!("/live/entity/{}", seed.entity_id))
        .await;
    assert!(reloaded.contains("Marker stone"));
    assert!(reloaded.contains(&seed.entity_id.to_string()));
    assert!(reloaded.contains(&format!("href=\"/live/user/{}\"", seed.user_id)));
}

#[sqlx::test(migrations = "../game/migration")]
async fn live_lists_state_loaded_row_filtering_and_honest_continuation(pool: PgPool) {
    seed(&pool).await;
    let server = StudioServer::start(pool).await;
    let html = server.html("/live/entity?limit=1").await;

    assert!(html.contains("Filtering applies only to the loaded rows."));
    assert!(html.contains("This page is truncated."));
    assert!(html.contains("Load more"));
    assert!(html.contains("limit=1"));
}

#[sqlx::test(migrations = "../game/migration")]
async fn activity_operation_filtering_is_client_only_and_not_continued(pool: PgPool) {
    seed(&pool).await;
    let server = StudioServer::start(pool).await;
    let html = server
        .html("/live/activity?operation=does_not_exist&limit=1")
        .await;

    assert!(html.contains("Operation filtering applies only to loaded rows"));
    assert!(html.contains("name=\"operation-filter\""));
    assert!(html.contains("Load more"));
    let load_more_end = html
        .find(">Load more</a>")
        .expect("the truncated page should render a Load more link");
    let load_more_start = html[..load_more_end]
        .rfind("href=\"")
        .expect("Load more should carry an href");
    let load_more = &html[load_more_start..load_more_end];
    assert!(load_more.contains("/live/activity?before_at="));
    assert!(!load_more.contains("operation="));
}

#[sqlx::test(migrations = "../game/migration")]
async fn a_table_without_a_primary_key_fails_closed_as_an_unavailable_row_view(pool: PgPool) {
    sqlx::query("CREATE TABLE studio_unpageable (value text NOT NULL)")
        .execute(&pool)
        .await
        .expect("the no-primary-key fixture should be created");
    let server = StudioServer::start(pool).await;
    let response = server
        .client
        .get(format!(
            "{}/live/storage/studio_unpageable",
            server.base_url
        ))
        .send()
        .await
        .expect("the row-view request should send");

    assert_eq!(response.status(), StatusCode::CONFLICT);
    let html = response.text().await.expect("the conflict should be HTML");
    assert!(html.contains("Rows cannot be browsed"));
    assert!(html.contains("fails closed"));
}

#[sqlx::test(migrations = "../game/migration")]
async fn unknown_live_records_and_tables_are_honest_not_found_pages(pool: PgPool) {
    let server = StudioServer::start(pool).await;
    for path in [
        format!("/live/entity/{}", Uuid::new_v4()),
        "/live/storage/not_a_table".to_owned(),
    ] {
        let response = server
            .client
            .get(format!("{}{}", server.base_url, path))
            .send()
            .await
            .expect("request should send");
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert!(response.text().await.expect("body").contains("Not found"));
    }
}
