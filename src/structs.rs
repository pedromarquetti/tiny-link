use crate::schema::tiny_link;

#[derive(Queryable, Serialize, Debug)]
pub struct LongUrl {
    pub long_url: String,
}
#[derive(Queryable, Serialize, Debug)]
pub struct ShortUrl {
    pub short_url: String,
}
#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = tiny_link)]
pub struct TinyLink {
    pub long_link: String,
    pub short_link: String,
}
