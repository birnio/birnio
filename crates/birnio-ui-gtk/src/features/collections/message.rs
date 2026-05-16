use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum Message {
    SelectCollection(Uuid),
    SelectRequest(Uuid),
    CreateRequest,
}
