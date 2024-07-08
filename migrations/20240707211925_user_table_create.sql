-- Add migration script here
-- Add up migration script here

-- 유저 테이블 시퀀스
CREATE SEQUENCE tb_user_seq
START WITH 1
INCREMENT BY 1
MINVALUE 1
MAXVALUE 9223372036854775807
CACHE 1;
COMMENT ON SEQUENCE tb_user_seq IS '유저테이블 시퀀스';

-- 유저 테이블
CREATE TABLE tb_user(
    sn INT PRIMARY KEY DEFAULT nextval('tb_user_seq'),
    nick_name VARCHAR(30) NOT NULL,
    login_ty_cd VARCHAR(15) NOT NULL,
    email VARCHAR(100) ,
    password VARCHAR(500),
    provider_id VARCHAR(100),
    user_stt_cd VARCHAR(15) NOT NULL,
    user_ty_cd VARCHAR(15) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by int NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_by int NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);

COMMENT ON TABLE tb_user IS '유저테이블';
COMMENT ON COLUMN tb_user.sn IS '식별자';
COMMENT ON COLUMN tb_user.nick_name IS '닉네임';
COMMENT ON COLUMN tb_user.login_ty_cd IS '로그인 타입';
COMMENT ON COLUMN tb_user.email IS '이메일';
COMMENT ON COLUMN tb_user.password IS '패스워드';
COMMENT ON COLUMN tb_user.provider_id IS '프로바이더아이디';
COMMENT ON COLUMN tb_user.user_stt_cd IS '상태코드';
COMMENT ON COLUMN tb_user.user_ty_cd IS '타입코드';
COMMENT ON COLUMN tb_user.created_at IS '생성일시';
COMMENT ON COLUMN tb_user.created_by IS '생성자';
COMMENT ON COLUMN tb_user.updated_at IS '수정일시';
COMMENT ON COLUMN tb_user.updated_by IS '수정자';
COMMENT ON COLUMN tb_user.is_deleted IS '삭제여부';

-- 유저테이블 인덱스
CREATE UNIQUE INDEX tb_user_uidx__nick_name ON tb_user (nick_name);
CREATE INDEX tb_user_idx__email__login_ty_cd__is_deleted ON tb_user (email, login_ty_cd, is_deleted);
COMMENT ON INDEX tb_user_uidx__nick_name IS '유저 닉네임 유니크 인덱스';
COMMENT ON INDEX tb_user_idx__email__login_ty_cd__is_deleted IS '유저 가입여부 인덱스';