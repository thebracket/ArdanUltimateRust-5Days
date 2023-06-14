CREATE TABLE IF NOT EXISTS timeseries
(
    id SERIAL PRIMARY KEY,
    collector_id VARCHAR(255),
    received TIMESTAMP,
    total_memory UNSIGNED BIG INT,
    used_memory UNSIGNED BIG INT,
    average_cpu FLOAT
)
