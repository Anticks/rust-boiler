-- Create "subscriptions" table
CREATE TABLE "subscriptions" (
  "id" uuid NOT NULL,
  "email" text NOT NULL,
  "name" text NOT NULL,
  "subscribed_at" timestamptz NOT NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "subscriptions_email_key" UNIQUE ("email")
);
