#[derive(Queryable, Serialize, Debug)]
pub struct TinyLink {
    pub id: i32,
    pub long_link: String,
    pub short_link: String,
}
