use birnio_core::Request;

#[derive(Clone, Debug)]
pub enum Command {
    SendRequest(Request),
    LoadCollections,
}
