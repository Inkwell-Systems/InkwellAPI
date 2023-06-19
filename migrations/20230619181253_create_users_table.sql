-- TODO(calco): Do not forget about the restrictions made to display_name
-- and email.
CREATE TABLE users(
    uid uuid NOT NULL PRIMARY KEY,
--     PRIMARY KEY(uid),
    
    display_name VARCHAR(50) NOT NULL,
    email VARCHAR(50) NOT NULL UNIQUE,
    profile_url TEXT,
    created_at DATE NOT NULL
)