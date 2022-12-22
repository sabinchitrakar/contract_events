use bonsaidb::{
    core::schema::{Collection, SerializedCollection},
    local::{
        config::{Builder, StorageConfiguration},
        Database,
    },
};

pub fn get_connection<T: bonsaidb::core::schema::Collection + 'static>(
    path: String,
) -> Result<Database, bonsaidb::core::Error> {
    let db = Database::open::<T>(StorageConfiguration::new(&path))?;
    Ok(db)
}
