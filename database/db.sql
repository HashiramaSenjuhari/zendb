/* Reference Schema */
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS elonmusk (
id UUID DEFAULT uuid_generate_v4() NOT NULL,
name DATE DEFAULT CURRENT_DATE,
time TIME DEFAULT CURRENT_TIME,
fingerprint TIMESTAMP DEFAULT NOW(),
age INT,
story TEXT NOT NULL DEFAULT "billionairehari",
billionaire BOOL,
PRIMARY KEY (id,story)
);
CREATE INDEX index_hmphQJ ON elonmusk (id);
CLUSTER elonmusk USING index_hmphQJ;
