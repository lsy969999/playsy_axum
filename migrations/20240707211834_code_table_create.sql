-- Add migration script here
-- 코드테이블
CREATE TABLE tb_code (
    code_id VARCHAR(30),
    code_value VARCHAR(15),
    code_value_nm VARCHAR(50),
    code_desc VARCHAR(100),

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by INT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_by INT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY(code_id, code_value)
);

-- 유저 상태 코드값
INSERT INTO tb_code
(code_id, code_value, code_value_nm, code_desc, created_by, updated_at, updated_by)
VALUES
('user_stt_cd', 'OK', '정상', '정상', 1, CURRENT_TIMESTAMP, 1);
INSERT INTO tb_code
(code_id, code_value, code_value_nm, code_desc, created_by, updated_at, updated_by)
VALUES
('user_stt_cd', 'QUIT', '탈퇴', '탈퇴', 1, CURRENT_TIMESTAMP, 1);

-- 유저 타입 코드값
INSERT INTO tb_code
(code_id, code_value, code_value_nm, code_desc, created_by, updated_at, updated_by)
VALUES
('user_ty_cd', 'USER', '유저', '유저', 1, CURRENT_TIMESTAMP, 1);
INSERT INTO tb_code
(code_id, code_value, code_value_nm, code_desc, created_by, updated_at, updated_by)
VALUES
('user_ty_cd', 'ADMIN', '관리자', '관리자', 1, CURRENT_TIMESTAMP, 1);

-- 로그인 타입 코드값
INSERT INTO tb_code
(code_id, code_value, code_value_nm, code_desc, created_by, updated_at, updated_by)
VALUES
('login_ty_cd', 'EMAIL', '이메일', '이메일', 1, CURRENT_TIMESTAMP, 1);