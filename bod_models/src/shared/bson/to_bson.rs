use bson::Document;

pub trait ToBson{
    fn to_bson(&self)-> Result<Document,bson::ser::Error>;
}