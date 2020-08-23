use crate::video::Video;
use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct NewpipeDB {
    pub res: Vec<Video>,
}

pub async fn new_newpipe_db() -> Result<NewpipeDB, anyhow::Error> {
    let conn = Connection::open("./newpipe.db")?;
    let mut stmt = conn.prepare(
        "
        SELECT streams.url, streams.title, streams.uploader 
        FROM stream_history 
        LEFT JOIN streams ON streams.uid=stream_history.stream_id 
        ORDER BY access_date DESC
        ",
    )?;

    let res = stmt
        .query_map(params![], |row| {
            Ok(Video {
                url: row.get(0)?,
                title: row.get(1)?,
                uploader: row.get(2)?,
            })
        })?
        .map(|r| r.unwrap())
        .collect::<Vec<Video>>();

    let db = NewpipeDB { res: res };

    Ok(db)
}
