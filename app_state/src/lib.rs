use auto_delegate::Delegate;
use clients::null_client::{self, NullClient};

struct RawAppState<N> {
    #[to(NullClient)]
    null_client: N,
}

pub trait AppState: NullClient {}

impl<N: NullClient> NullClient for RawAppState<N> {
    fn do_nothing(&self) { self.null_client.do_nothing() }
}

impl<N: NullClient> AppState for RawAppState<N> {}

pub fn create_app_state() -> impl AppState {
    RawAppState {
        null_client: null_client::NullClientImpl {},
    }
}
