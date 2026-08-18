What it does:
creates one unplaced shared Entity with its initial Properties and Traits, atomically with its Activity.

Use it when:
only if later participants must be able to refer to the same subject again, and only after the User explicitly confirmed the whole previewed package.

Input meaning:
English name and description, each Property key and each Trait statement used once. It asserts no creation, ownership or discovery in the story; calling it again creates another Entity.

After acceptance:
render only the established named subject and its accepted qualities.

Never:
place the Entity through this call, let the User edit storage directly, or expose ids or who controls what. No background process runs.
