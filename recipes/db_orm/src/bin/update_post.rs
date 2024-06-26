use migration::DbErr;
use sea_orm::{EntityTrait, Set, ActiveModelTrait};
use db_orm::establish_connection;
use entity::post;

#[tokio::main]
async fn main() -> Result<(), DbErr>{
    let db = establish_connection().await?;

    // UPDATE titel of Post by ID
    let post = post::Entity::find_by_id(1).one(&db).await?;
    let mut post: post::ActiveModel = post.unwrap().into();
    post.title = Set("Updated title".to_owned());
    let post: post::Model = post.update(&db).await?;

    println!("Post updated for ID: {} with TITLE: {}", post.id, post.title);

    Ok(())
}
