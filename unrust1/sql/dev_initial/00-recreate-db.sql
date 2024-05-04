-- DEV ONLY - Brute Force DROP DB (for loval dev and unit test)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE usename = 'gulp_dev' OR datname  = 'gulp_inventory';
DROP DATABASE IF EXISTS gulp_inventory;
DROP USER IF EXISTS gulp_dev;

-- DEV ONLY - Dev only passsword (for local dev and unit test).
CREATE USER gulp_dev PASSWORD 'gulp_dev_only_pwd';
CREATE DATABASE gulp_inventory owner gulp_dev ENCODING = 'UTF-8';
