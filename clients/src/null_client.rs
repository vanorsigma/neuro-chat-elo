pub trait NullClient {
    fn do_nothing(&self);
}
pub struct NullClientImpl;

impl NullClient for NullClientImpl {
    fn do_nothing(&self) {}
}
