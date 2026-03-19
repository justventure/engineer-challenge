CREATE TYPE settings_method AS ENUM (
    'password',
    'profile',
    'lookup_secret',
    'totp'
);

CREATE TABLE settings_audit (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    method     settings_method NOT NULL,
    ip_address VARCHAR(45),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_settings_audit_user_id ON settings_audit(user_id);
