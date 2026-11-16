use toasty::{Db, schema::Model, stmt::CreateMany};

// PostgreSQLのバインド上限/メッセージサイズ超過を避けるため分割登録する
pub const INSERT_CHUNK_SIZE: usize = 2000;

pub async fn batch_insert<M, T, F>(
    db: &mut Db,
    items: &[T],
    make: fn() -> CreateMany<M>,
    mut map: F,
) -> toasty::Result<()>
where
    M: Model,
    F: FnMut(&T, M::Create) -> M::Create,
{
    for chunk in items.chunks(INSERT_CHUNK_SIZE) {
        let mut query = make();
        for item in chunk {
            query = query.with_item(|b| map(item, b));
        }
        query.exec(db).await?;
    }
    Ok(())
}
