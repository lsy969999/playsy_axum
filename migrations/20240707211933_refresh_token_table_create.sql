-- Add migration script here

-- 새로고침토큰 테이블 시퀀스
CREATE SEQUENCE tb_refresh_token_seq
START WITH 1
INCREMENT BY 1
MINVALUE 1
MAXVALUE 9223372036854775807
CACHE 1;
COMMENT ON SEQUENCE tb_refresh_token_seq IS '새로고침토큰 테이블 시퀀스';

-- 새로고침 토큰 테이블
CREATE TABLE tb_refresh_token(
    sn INT PRIMARY KEY DEFAULT nextval('tb_refresh_token_seq'),
    user_sn INT NOT NULL,
    hash VARCHAR(200) NOT NULL,
    refresh_token VARCHAR(500) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    forwarded_ip VARCHAR(100) NULL,
    client_ip VARCHAR(100) NOT NULL,
    user_agent VARCHAR(300) NOT NULL,
    --
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by INT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_by INT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);
COMMENT ON TABLE tb_refresh_token IS '새로고침토큰 테이블';
COMMENT ON COLUMN tb_refresh_token.sn IS '식별자';
COMMENT ON COLUMN tb_refresh_token.user_sn IS '사용자 식별자';
COMMENT ON COLUMN tb_refresh_token.hash IS '해시';
COMMENT ON COLUMN tb_refresh_token.refresh_token IS '새로고침토큰';
COMMENT ON COLUMN tb_refresh_token.expires_at IS '만료일시';
COMMENT ON COLUMN tb_refresh_token.forwarded_ip IS 'forwardedIP';
COMMENT ON COLUMN tb_refresh_token.client_ip IS '클라이언트IP';
COMMENT ON COLUMN tb_refresh_token.user_agent IS '유저에이전트';
COMMENT ON COLUMN tb_refresh_token.created_at IS '생성일시';
COMMENT ON COLUMN tb_refresh_token.created_by IS '생성자';
COMMENT ON COLUMN tb_refresh_token.updated_at IS '수정일시';
COMMENT ON COLUMN tb_refresh_token.updated_by IS '수정자';
COMMENT ON COLUMN tb_refresh_token.is_deleted IS '삭제여부';

-- 새로고침 토큰 테이블 인덱스
CREATE INDEX tb_refresh_token_idx__user_sn__hash__is_deleted ON tb_refresh_token (user_sn, hash, is_deleted);
COMMENT ON INDEX tb_refresh_token_idx__user_sn__hash__is_deleted IS '새로고침토큰 조회 인덱스';