CREATE TYPE token_purpose AS ENUM (
    'email_verification',
    'password_recovery'
);

CREATE TABLE verification_tokens (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token      VARCHAR(512) NOT NULL UNIQUE,
    code       VARCHAR(16),
    purpose    token_purpose NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at    TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_verification_tokens_token   ON verification_tokens(token);
CREATE INDEX idx_verification_tokens_code    ON verification_tokens(code);
CREATE INDEX idx_verification_tokens_user_id ON verification_tokens(user_id);
CREATE INDEX idx_verification_tokens_purpose ON verification_tokens(purpose);
