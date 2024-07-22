-- Add migration script here
-- Add up migration script here

-- 유저 상태 ENUM 타입 정의
CREATE TYPE USER_STT_ENUM AS ENUM ('WAIT_EMAIL_VERI', 'OK', 'QUIT');

-- 유저 타입 ENUM 타입 정의
CREATE TYPE USER_TY_ENUM AS ENUM ('USER', 'ADMIN');

-- 로그인 타입 ENUM 타입 정의
CREATE TYPE PROVIDER_TY_ENUM AS ENUM ('EMAIL', 'GOOGLE', 'KAKAO', 'NAVER', 'GITHUB', 'APPLE', 'FACEBOOK');

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
    avatar_url VARCHAR(255) NULL,
    nick_name VARCHAR(30) NOT NULL,
    email VARCHAR(100) NOT NULL,
    password VARCHAR(500) NULL,
    provider_ty_enum PROVIDER_TY_ENUM NOT NULL,
    provider_id VARCHAR(100) NULL,
    provider_secret VARCHAR(100) NULL,
    provider_access_token VARCHAR(300) NULL,
    provider_refresh_token VARCHAR(300) NULL,
    user_stt_enum USER_STT_ENUM NOT NULL,
    user_ty_enum USER_TY_ENUM NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by INT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_by INT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);

COMMENT ON TABLE tb_user IS '유저테이블';
COMMENT ON COLUMN tb_user.sn IS '식별자';
COMMENT ON COLUMN tb_user.avatar_url IS '아바타url';
COMMENT ON COLUMN tb_user.nick_name IS '닉네임';
COMMENT ON COLUMN tb_user.email IS '이메일';
COMMENT ON COLUMN tb_user.password IS '패스워드';
COMMENT ON COLUMN tb_user.provider_ty_enum IS '프로바이더타입';
COMMENT ON COLUMN tb_user.provider_id IS '프로바이더아이디';
COMMENT ON COLUMN tb_user.provider_secret IS '프로바이더시크릿';
COMMENT ON COLUMN tb_user.provider_access_token IS '프로바이더액세스토큰';
COMMENT ON COLUMN tb_user.provider_refresh_token IS '프로바이더리프레시토큰';
COMMENT ON COLUMN tb_user.user_stt_enum IS '상태코드';
COMMENT ON COLUMN tb_user.user_ty_enum IS '타입코드';
COMMENT ON COLUMN tb_user.created_at IS '생성일시';
COMMENT ON COLUMN tb_user.created_by IS '생성자';
COMMENT ON COLUMN tb_user.updated_at IS '수정일시';
COMMENT ON COLUMN tb_user.updated_by IS '수정자';
COMMENT ON COLUMN tb_user.is_deleted IS '삭제여부';

-- 유저테이블 인덱스
CREATE UNIQUE INDEX tb_user_uidx__nick_name ON tb_user (nick_name);
CREATE INDEX tb_user_idx__email__provider_ty_enum__is_deleted ON tb_user (email, provider_ty_enum, is_deleted);
COMMENT ON INDEX tb_user_uidx__nick_name IS '유저 닉네임 유니크 인덱스';
COMMENT ON INDEX tb_user_idx__email__provider_ty_enum__is_deleted IS '유저 가입여부 인덱스';

--- 유저 이메일 인증 시퀀스
CREATE SEQUENCE tb_email_join_verifications_seq
START WITH 1
INCREMENT BY 1
MINVALUE 1
MAXVALUE 9223372036854775807
CACHE 1;
COMMENT ON SEQUENCE tb_email_join_verifications_seq IS '이메일가입 인증 시퀀스';

CREATE TABLE tb_email_join_verifications(
    sn INT PRIMARY KEY DEFAULT nextval('tb_email_join_verifications_seq'),
    user_sn INT NOT NULL,
    code VARCHAR(15) NOT NULL,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by INT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_by INT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);

COMMENT ON TABLE tb_email_join_verifications IS '이메일가입 인증 테이블';
COMMENT ON COLUMN tb_email_join_verifications.sn IS '식별자';
COMMENT ON COLUMN tb_email_join_verifications.user_sn IS '유저 식별자';
COMMENT ON COLUMN tb_email_join_verifications.code IS '코드';
COMMENT ON COLUMN tb_email_join_verifications.is_verified IS '인증완료여부';
COMMENT ON COLUMN tb_email_join_verifications.expires_at IS '만료일';
COMMENT ON COLUMN tb_email_join_verifications.created_at IS '생성일시';
COMMENT ON COLUMN tb_email_join_verifications.created_by IS '생성자';
COMMENT ON COLUMN tb_email_join_verifications.updated_at IS '수정일시';
COMMENT ON COLUMN tb_email_join_verifications.updated_by IS '수정자';
COMMENT ON COLUMN tb_email_join_verifications.is_deleted IS '삭제여부';

CREATE INDEX tb_email_join_verifications_idx__code__is_deleted ON tb_email_join_verifications (code, is_deleted);
COMMENT ON INDEX tb_email_join_verifications_idx__code__is_deleted IS '이메일가입 인증 테이블 코드 중복체크용 인덱스';