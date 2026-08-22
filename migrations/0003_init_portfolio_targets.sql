-- Migração 0003: Metas de Alocação por Classe de Ativos
CREATE TABLE IF NOT EXISTS class_targets (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    asset_class VARCHAR(50) NOT NULL,
    target_percentage DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uq_user_class UNIQUE (user_id, asset_class)
);

CREATE INDEX IF NOT EXISTS idx_class_targets_user ON class_targets(user_id);
