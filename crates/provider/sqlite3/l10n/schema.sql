--
-- File generated with SQLiteStudio v3.4.4 on Thu Nov 14 15:44:01 2024
--
-- Text encoding used: UTF-8
--
PRAGMA foreign_keys = off;
BEGIN TRANSACTION;

-- Table: component
CREATE TABLE IF NOT EXISTS component (rowID INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, identifier TEXT UNIQUE NOT NULL, languageTag TEXT REFERENCES language (tag) ON DELETE RESTRICT ON UPDATE NO ACTION NOT NULL, comment TEXT, added DATE NOT NULL);

-- Table: contributor
CREATE TABLE IF NOT EXISTS contributor (rowID INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, component TEXT NOT NULL REFERENCES component (identifier) ON DELETE RESTRICT ON UPDATE NO ACTION, languageTag TEXT NOT NULL REFERENCES language (tag) ON DELETE RESTRICT ON UPDATE NO ACTION, contributor TEXT NOT NULL, substituteFor TEXT, comment TEXT, verified DATE, UNIQUE (component, languageTag, contributor));

-- Table: language
CREATE TABLE IF NOT EXISTS language (rowID INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, tag TEXT UNIQUE NOT NULL, englishName TEXT, added DATE NOT NULL);

-- Table: languageData
CREATE TABLE IF NOT EXISTS languageData (rowID INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, component TEXT NOT NULL REFERENCES component (identifier) ON DELETE RESTRICT ON UPDATE NO ACTION, languageTag TEXT NOT NULL REFERENCES language (tag) ON DELETE RESTRICT ON UPDATE NO ACTION, count INTEGER NOT NULL, ratio REAL NOT NULL, UNIQUE (component, languageTag));

-- Table: metadata
CREATE TABLE IF NOT EXISTS metadata (rowID INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, component TEXT NOT NULL REFERENCES component (identifier) ON DELETE RESTRICT ON UPDATE NO ACTION, key TEXT NOT NULL, value TEXT NOT NULL, comment TEXT, verified DATE, UNIQUE (component, key));

-- Table: pattern
CREATE TABLE IF NOT EXISTS pattern (rowID INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, component TEXT NOT NULL REFERENCES component (identifier) ON DELETE RESTRICT ON UPDATE NO ACTION, identifier TEXT NOT NULL, languageTag TEXT NOT NULL REFERENCES language (tag) ON DELETE RESTRICT ON UPDATE NO ACTION, string TEXT NOT NULL, comment TEXT, verified DATE, CONSTRAINT u_IdentierLanguage UNIQUE (component, identifier, languageTag));

COMMIT TRANSACTION;
PRAGMA foreign_keys = on;
