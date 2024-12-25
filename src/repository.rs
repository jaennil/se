pub struct Repository {
    db: rusqlite::Connection,
}

impl Repository {
    pub fn new(db: rusqlite::Connection) -> Self {
        Self {
            db
        }
    }
}
