ALTER TABLE app_user RENAME TO "user";

ALTER TABLE "user" RENAME CONSTRAINT app_user_pkey TO user_pkey;
