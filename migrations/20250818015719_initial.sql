-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE product (
    product_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_name TEXT NOT NULL UNIQUE
);
