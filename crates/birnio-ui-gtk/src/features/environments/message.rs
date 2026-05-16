use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum Message {
    SelectEnvironment(Option<Uuid>),
}
