-- Create the table
CREATE TABLE monitored_websites (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    check_interval INT DEFAULT 5, -- Check interval in minutes
    latency_threshold INT DEFAULT 200, -- Latency threshold in milliseconds
    consecutive_failures_threshold INT DEFAULT 3, -- Failure threshold before alert
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create a function to automatically update the `updated_at` column
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to use the above function
CREATE TRIGGER set_updated_at
BEFORE UPDATE ON monitored_websites
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
