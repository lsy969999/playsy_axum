-- Add migration script here
-- 문의 테이블 시퀀스
CREATE SEQUENCE tb_inquery_seq
START WITH 1
INCREMENT BY 1
MINVALUE 1
MAXVALUE 9223372036854775807
CACHE 1;
COMMENT ON SEQUENCE tb_inquery_seq IS '문의 테이블 시퀀스';

CREATE TABLE tb_inquery(
    sn INT PRIMARY KEY DEFAULT nextval('tb_inquery_seq'),
    user_sn INT NULL,
    email VARCHAR(100) NULL,
    subject VARCHAR(255) NULL,
    message TEXT NULL,
    answered_at TIMESTAMP WITH TIME ZONE NULL,
    answer TEXT NULL,
    --
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by INT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_by INT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);
COMMENT ON TABLE tb_inquery IS '문의 테이블';
COMMENT ON COLUMN tb_inquery.sn IS '식별자';
COMMENT ON COLUMN tb_inquery.user_sn IS '사용자 식별자';
COMMENT ON COLUMN tb_inquery.subject IS '주제';
COMMENT ON COLUMN tb_inquery.message IS '메시지';
COMMENT ON COLUMN tb_inquery.answered_at IS '답변일시';
COMMENT ON COLUMN tb_inquery.answer IS '답변';
COMMENT ON COLUMN tb_inquery.created_at IS '생성일시';
COMMENT ON COLUMN tb_inquery.created_by IS '생성자';
COMMENT ON COLUMN tb_inquery.updated_at IS '수정일시';
COMMENT ON COLUMN tb_inquery.updated_by IS '수정자';
COMMENT ON COLUMN tb_inquery.is_deleted IS '삭제여부';