-- Add migration script here

-- 공지사항 테이블 시퀀스
CREATE SEQUENCE tb_announcement_seq
START WITH 1
INCREMENT BY 1
MINVALUE 1
MAXVALUE 9223372036854775807
CACHE 1;
COMMENT ON SEQUENCE tb_announcement_seq IS '공지사항 테이블 시퀀스';

-- 공지사항 토큰 테이블
CREATE TABLE tb_announcement(
    sn INT PRIMARY KEY DEFAULT nextval('tb_announcement_seq'),
    user_sn INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    --
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by INT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_by INT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);
COMMENT ON TABLE tb_announcement IS '새로고침토큰 테이블';
COMMENT ON COLUMN tb_announcement.sn IS '식별자';
COMMENT ON COLUMN tb_announcement.user_sn IS '사용자 식별자';
COMMENT ON COLUMN tb_announcement.title IS '제목';
COMMENT ON COLUMN tb_announcement.content IS '컨텐츠';
COMMENT ON COLUMN tb_announcement.created_at IS '생성일시';
COMMENT ON COLUMN tb_announcement.created_by IS '생성자';
COMMENT ON COLUMN tb_announcement.updated_at IS '수정일시';
COMMENT ON COLUMN tb_announcement.updated_by IS '수정자';
COMMENT ON COLUMN tb_announcement.is_deleted IS '삭제여부';