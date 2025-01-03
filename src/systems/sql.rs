pub fn sql_schema() -> String {
    let string = "
        -- Table where each row represents a picture.
        CREATE TABLE IF NOT EXISTS pictures (
            id                      INTEGER NOT NULL UNIQUE,
            filename                TEXT NOT NULL,
            thumbnail               TEXT,
            PRIMARY KEY(id AUTOINCREMENT)
        );
    ";

    string.to_string()
}
