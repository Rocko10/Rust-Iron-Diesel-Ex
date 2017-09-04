#[derive(Queryable)]
#[derive(Debug)]
pub struct Monkey {

    pub id: i32,
    pub name: String,
    pub color: Option<String>

}
