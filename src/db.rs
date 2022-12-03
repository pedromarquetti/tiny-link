use crate::structs::{ShortUrl, TinyLink};

use diesel::prelude::*;
use diesel::{pg::PgConnection, result::Error as db_err};

/// writes received long URL to db, returns short Url that will be echoed to user
pub fn write_to_db(
    recvd_long_url: String,
    db_connection: &mut PgConnection,
) -> Result<String, db_err> {
    // TODO:
    // 1. Implement duplicate check
    use crate::schema::tiny_link;
    use rand::{distributions::Alphanumeric, Rng};

    let rand: String = rand::thread_rng() // generating random String to be used as short url
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    return diesel::insert_into(tiny_link::table)
        // inserting TinyLink with long + short url
        .values(&TinyLink {
            long_link: recvd_long_url,
            short_link: rand, // this has to be a short (6) random id
                              // the server doesn't check for duplicates, yet
        })
        .returning(tiny_link::short_link) // returns short url to user
        .get_result(db_connection);

    // let result: Result<String, db_err> = diesel::insert_into(tiny_link::table)
    //     // inserting TinyLink with long + short url
    //     .values(&TinyLink {
    //         long_link: recvd_long_url,
    //         short_link: rand, // this has to be a short (6) random id
    //                           // the server doesn't check for duplicates, yet
    //     })
    //     .returning(tiny_link::short_link) // returns short url to user
    //     .get_result(db_connection);

    // match result {
    //     Ok(shortened) => shortened, // all ok, sending back short url

    //     Err(error) => {
    //         // something happened, creating error
    //         error!("Error writing to database: {}", error.to_string());
    //         format!("DB Error: {}", error);
    //     }
    // };
}

pub fn read_from_db(path: ShortUrl, db_connection: &mut PgConnection) -> Option<TinyLink> {
    use crate::schema::tiny_link::{long_link, short_link, table};

    let query = table
        .select(long_link)
        .filter(short_link.eq(&path.short_url))
        .first::<String>(db_connection);
    match query {
        Ok(success_res) => Some(TinyLink {
            long_link: success_res,
            short_link: path.short_url,
        }),
        Err(error) => {
            error!("Query Error: {}", error);
            None
        }
    }
}
