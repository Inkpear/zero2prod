-- Add migration script here
-- //password = cd2117c2-98a6-4403-822c-e83727a64eb4
INSERT INTO users (user_id, username, password_hash)
VALUES (
    'ddf8994f-d522-4659-8d02-c1d479057be6',
    'admin',
    '$argon2id$v=19$m=4096,t=3,p=1$I/4LK4zp59my4dq1JjHgdA$lDV2yWx2RtF4iKG3rzjmhL2KyMukfQUTdON3XWKaZcc'
);