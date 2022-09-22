use crate::structs::LongUrl;

use futures::future::FutureResult;
use hyper::Error;

/// writes received long URL to db, returns short Url that will be echoed to user
pub fn write_to_db(entry: LongUrl) -> FutureResult<LongUrl, Error> {
    // TODO: save 'entry' to db...

    futures::future::ok(entry)
}
