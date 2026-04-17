CREATE TABLE IF NOT EXISTS licenses (
    license_id TEXT PRIMARY KEY,
    purchaser_email TEXT NOT NULL,
    plan_tier TEXT NOT NULL,
    entitlement_status TEXT NOT NULL,
    issue_timestamp TEXT NOT NULL,
    expiration_timestamp TEXT,
    activation_limit INTEGER NOT NULL,
    created_timestamp TEXT NOT NULL,
    updated_timestamp TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS payment_events (
    internal_event_id TEXT PRIMARY KEY,
    payment_provider_identifier TEXT NOT NULL,
    provider_event_identifier TEXT NOT NULL,
    event_type TEXT NOT NULL,
    receipt_timestamp TEXT NOT NULL,
    processing_status TEXT NOT NULL,
    payload_reference TEXT,
    related_license_id TEXT,
    purchaser_email TEXT NOT NULL,
    plan_tier TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_payment_events_provider_event
ON payment_events (payment_provider_identifier, provider_event_identifier);

CREATE TABLE IF NOT EXISTS device_activations (
    activation_id TEXT PRIMARY KEY,
    parent_license_id TEXT NOT NULL,
    activation_slot_index INTEGER NOT NULL,
    device_hash TEXT NOT NULL,
    client_platform_identity TEXT NOT NULL,
    activation_status TEXT NOT NULL,
    first_activation_timestamp TEXT NOT NULL,
    last_confirmed_timestamp TEXT,
    created_timestamp TEXT NOT NULL,
    updated_timestamp TEXT NOT NULL,
    FOREIGN KEY (parent_license_id) REFERENCES licenses (license_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_device_activations_active_slot
ON device_activations (parent_license_id, activation_slot_index)
WHERE activation_status = 'active';

CREATE UNIQUE INDEX IF NOT EXISTS idx_device_activations_active_device
ON device_activations (parent_license_id, device_hash)
WHERE activation_status = 'active';

CREATE TABLE IF NOT EXISTS activation_audits (
    audit_id TEXT PRIMARY KEY,
    related_license_id TEXT,
    related_activation_id TEXT,
    action_type TEXT NOT NULL,
    actor_type TEXT NOT NULL,
    reason_classification TEXT NOT NULL,
    metadata_reference TEXT,
    recorded_timestamp TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS token_issuances (
    token_issuance_id TEXT PRIMARY KEY,
    parent_license_id TEXT NOT NULL,
    parent_activation_id TEXT NOT NULL,
    issuance_timestamp TEXT NOT NULL,
    expiration_timestamp TEXT,
    key_identifier TEXT NOT NULL,
    token_fingerprint TEXT NOT NULL,
    token_status TEXT NOT NULL,
    FOREIGN KEY (parent_license_id) REFERENCES licenses (license_id),
    FOREIGN KEY (parent_activation_id) REFERENCES device_activations (activation_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_token_issuances_active_activation
ON token_issuances (parent_activation_id)
WHERE token_status = 'active';