use crate::schema::tiny_link;

#[derive(Queryable, Serialize, Debug)]
pub struct LongUrl {
    pub long_url: String,
}

#[derive(Insertable, Debug)]
#[table_name = "tiny_link"]
pub struct TinyLink {
    pub long_link: String,
    pub short_link: String,
}
