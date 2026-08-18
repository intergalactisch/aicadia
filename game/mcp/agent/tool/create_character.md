What it does:
creates the current User's one Character — unplaced — with its initial Properties and Traits, atomically with its Activity.

Use it when:
only after get_character returned character_not_found and the User explicitly confirmed the whole previewed package.

Input meaning:
it derives the User and accepts no ids; English name and description, each Property key and each Trait statement used once.

After acceptance:
describe the named person and their accepted qualities — and that they have not entered the World yet.

Never:
recreate an existing Character, let the User edit stored state directly, or expose ids or who controls what. No background process runs.
