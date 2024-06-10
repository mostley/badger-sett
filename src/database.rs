use crate::{BadgerDB, Error, Result};
use rocket::futures;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::{sqlx, Connection};
use sqlx::Acquire;

use futures::{future::TryFutureExt, stream::TryStreamExt};

#[derive(Serialize, Deserialize)]
pub struct Member {
    pub fob_id: String,
    // #[field(validate = len(..1024))]
    pub name: String,
    // #[field(validate = len(..1024))]
    pub contact_data: String,
}

fn parse_fob_id(id_str: &str) -> Result<[u8; 4]> {
    let mut id_bytes = [0u8; 4];

    hex::decode_to_slice(id_str, &mut id_bytes as &mut [u8]).map_err(|_| {
        Error::BadRequest("Invalid fob_id, should be formatted as a 4 byte hex".into())
    })?;

    Ok(id_bytes)
}

pub async fn get_member_by_id(
    db: &mut Connection<crate::BadgerDB>,
    fob_id: String,
) -> Result<Member> {
    let tag_number = &parse_fob_id(&fob_id)?[..];

    let member = sqlx::query!(
        "SELECT Tag, Name, Comment FROM Tags WHERE Tag = ?",
        tag_number
    )
    .fetch_one(&mut ***db)
    .await?;

    Ok(Member {
        fob_id: hex::encode(tag_number),
        name: member.Name.unwrap(),
        contact_data: member.Comment.unwrap(),
    })
}

pub async fn has_member_by_id(
    db: &mut Connection<crate::BadgerDB>,
    fob_id: String,
) -> Result<bool> {
    let member_result = get_member_by_id(db, fob_id).await;
    match member_result {
        Ok(_) => Ok(true),
        Err(Error::DBError(sqlx::Error::RowNotFound)) => Ok(false),
        Err(error) => Err(error),
    }
}

pub async fn create_member(
    db: &mut Connection<crate::BadgerDB>,
    new_member: Member,
) -> Result<Member> {
    let tag_number = &parse_fob_id(&new_member.fob_id)?[..];

    let results = sqlx::query!(
        "INSERT INTO Tags (Tag, Name, Comment) VALUES (?, ?, ?)",
        tag_number,
        new_member.name,
        new_member.contact_data
    )
    .fetch(&mut ***db)
    .try_collect::<Vec<_>>()
    .await?;

    Ok(new_member)
}

pub async fn update_member(
    db: &mut Connection<crate::BadgerDB>,
    updated_member: Member,
) -> Result<Member> {
    // Make sure member exists
    get_member_by_id(db, updated_member.fob_id.clone()).await?;

    let tag_number = &parse_fob_id(&updated_member.fob_id)?[..];

    let results = sqlx::query!(
        "UPDATE Tags SET Name = ?, Comment = ? WHERE Tag = ?",
        updated_member.name,
        updated_member.contact_data,
        tag_number,
    )
    .fetch(&mut ***db)
    .try_collect::<Vec<_>>()
    .await?;

    Ok(updated_member)
}

pub async fn delete_member(
    db: &mut Connection<crate::BadgerDB>,
    fob_id: String,
) -> Result<()> {
    // Make sure member exists
    get_member_by_id(db, fob_id.clone()).await?;

    let tag_number = &parse_fob_id(&fob_id)?[..];

    let results = sqlx::query!(
        "DELETE FROM Tags WHERE Tag = ?",
        tag_number
    )
    .fetch(&mut ***db)
    .try_collect::<Vec<_>>()
    .await?;

    Ok(())
}
