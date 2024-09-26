use std::sync::Arc;

pub trait AuthoredMesasge {
    type Id;

    fn author_id(&self) -> Self::Id;
}

impl<M> AuthoredMesasge for Arc<M>
where
    M: AuthoredMesasge,
{
    type Id = M::Id;

    fn author_id(&self) -> Self::Id {
        self.as_ref().author_id()
    }
}
