-- Add migration script here
CREATE TABLE wb_user (
    uid BIGSERIAL PRIMARY KEY,
    nickname VARCHAR(60) NOT NULL,
    phone VARCHAR(11) NOT NULL UNIQUE,
    password_hash VARCHAR(97) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE wb_follower (
    id BIGSERIAL PRIMARY KEY,  -- 自增主键，唯一标识每条记录
    follower_id BIGINT NOT NULL REFERENCES wb_user(uid),  -- 关注者ID，外键关联 wb_user 表
    followee_id BIGINT NOT NULL REFERENCES wb_user(uid),  -- 被关注者ID，外键关联 wb_user 表
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,  -- 记录创建时间，自动填充当前时间戳
    is_deleted BOOLEAN DEFAULT FALSE,  -- 标记是否取消关注（软删除）
    UNIQUE (follower_id, followee_id)  -- 复合唯一约束，避免重复关注关系
);

CREATE TABLE wb_post (
    pid BIGSERIAL PRIMARY KEY,
    uid BIGSERIAL NOT NULL REFERENCES wb_user(uid),
    content TEXT NOT NULL,
    is_deleted BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE wb_feed (
    fid BIGSERIAL PRIMARY KEY,
    pid BIGSERIAL NOT NULL REFERENCES wb_post(pid),
    uid BIGSERIAL NOT NULL REFERENCES wb_user(uid),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE wb_comment (
    cid BIGSERIAL PRIMARY KEY,
    pid BIGSERIAL NOT NULL REFERENCES wb_post(pid),
    uid BIGSERIAL NOT NULL REFERENCES wb_user(uid),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE wb_like (
    lid BIGSERIAL PRIMARY KEY,
    pid BIGSERIAL NOT NULL REFERENCES wb_post(pid) UNIQUE,
    uid BIGSERIAL NOT NULL REFERENCES wb_user(uid) UNIQUE
);

CREATE INDEX idx_follower_follower_id ON wb_follower(follower_id) WHERE is_deleted = FALSE;  -- 加速查询有效关注者
CREATE INDEX idx_follower_followee_id ON wb_follower(followee_id) WHERE is_deleted = FALSE;  -- 加速查询有效被关注者
