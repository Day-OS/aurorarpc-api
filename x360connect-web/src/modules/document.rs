use rocket_db_pools::mongodb::bson;
use rocket_db_pools::mongodb::bson::doc;

pub trait Document {
    fn database_name(&self) -> String;
    fn collection_name(&self) -> String;
    fn id(&self) -> bson::oid::ObjectId;
}

pub async fn save<T>(
    doc: &T,
    db: &rocket_db_pools::mongodb::Client,
) -> anyhow::Result<()>
where
    T: Document + serde::Serialize + Unpin + Sync,
{
    let filter = doc! { "_id": doc.id() };
    db.database(&doc.database_name())
        .collection::<T>(&doc.collection_name())
        .replace_one(filter, doc, None)
        .await?;

    Ok(())
}

pub async fn new<T>(
        doc: &T,
        db: &rocket_db_pools::mongodb::Client,
        
    ) -> anyhow::Result<()>
where
    T: Document + serde::Serialize + Unpin + Sync,
    {

        db.database(&doc.database_name())
            .collection::<T>(&doc.collection_name())
            .insert_one(doc, None)
            .await?;

        Ok(())
    }
