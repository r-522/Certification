-- =============================================================
-- TBL_USER: ユーザー管理
-- カラム命名: use(前方3文字) + 属性2文字 = 計5文字
-- =============================================================
CREATE TABLE IF NOT EXISTS "TBL_USER" (
    useid UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    usenm VARCHAR(100) NOT NULL,
    useml VARCHAR(64)  NOT NULL UNIQUE,  -- HMAC-SHA256(email) = 64 hex chars
    usepw VARCHAR(255) NOT NULL,          -- Argon2id hash
    usecr TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    useup TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- =============================================================
-- TBL_SESSION: セッション管理 (永続セッション)
-- カラム命名: ses(前方3文字) + 属性2文字
-- =============================================================
CREATE TABLE IF NOT EXISTS "TBL_SESSION" (
    sesid UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    sesus UUID        NOT NULL REFERENCES "TBL_USER"(useid) ON DELETE CASCADE,
    sesto VARCHAR(64) NOT NULL UNIQUE,   -- SHA-256(token) = 64 hex chars
    sescr TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_session_sesto ON "TBL_SESSION"(sesto);
CREATE INDEX IF NOT EXISTS idx_session_sesus ON "TBL_SESSION"(sesus);

-- =============================================================
-- TBL_CATALOG: 資格マスタ (入力補完用)
-- カラム命名: cat(前方3文字) + 属性2文字
-- =============================================================
CREATE TABLE IF NOT EXISTS "TBL_CATALOG" (
    catid UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    catnm VARCHAR(200) NOT NULL UNIQUE,
    catcr TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- =============================================================
-- TBL_CERTIFICATION: ユーザー保有資格
-- カラム命名: cer(前方3文字) + 属性2文字
-- =============================================================
CREATE TABLE IF NOT EXISTS "TBL_CERTIFICATION" (
    cerid UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    cerus UUID         NOT NULL REFERENCES "TBL_USER"(useid) ON DELETE CASCADE,
    cernm VARCHAR(200) NOT NULL,
    cerdt DATE,                            -- 取得日 (任意)
    cerno VARCHAR(500),                    -- メモ (note)
    cercr TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    cerup TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_cert_cerus ON "TBL_CERTIFICATION"(cerus);

-- =============================================================
-- TBL_GOAL: 資格取得目標
-- カラム命名: gol(前方3文字) + 属性2文字
-- =============================================================
CREATE TABLE IF NOT EXISTS "TBL_GOAL" (
    golid UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    golus UUID         NOT NULL REFERENCES "TBL_USER"(useid) ON DELETE CASCADE,
    golnm VARCHAR(200) NOT NULL,
    goldt DATE,                            -- 目標取得日 (任意)
    golst VARCHAR(20)  NOT NULL DEFAULT 'active'
              CHECK (golst IN ('active', 'achieved', 'abandoned')),
    golno VARCHAR(500),                    -- メモ (note)
    golcr TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    golup TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_goal_golus ON "TBL_GOAL"(golus);

-- =============================================================
-- TBL_FAVORITE: お気に入りユーザー
-- カラム命名: fav(前方3文字) + 属性2文字
-- =============================================================
CREATE TABLE IF NOT EXISTS "TBL_FAVORITE" (
    favid UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    favus UUID        NOT NULL REFERENCES "TBL_USER"(useid) ON DELETE CASCADE,  -- お気に入りした人
    favtg UUID        NOT NULL REFERENCES "TBL_USER"(useid) ON DELETE CASCADE,  -- お気に入りされた人
    favcr TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(favus, favtg),
    CHECK(favus != favtg)
);
CREATE INDEX IF NOT EXISTS idx_fav_favus ON "TBL_FAVORITE"(favus);
