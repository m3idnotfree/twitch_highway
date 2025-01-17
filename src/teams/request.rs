use crate::{
    base::{IntoQueryPairs, QueryParams},
    types::{constants::ID, Id},
};

#[derive(Debug)]
pub struct TeamFilter {
    name: Option<String>,
    id: Option<Id>,
}

impl TeamFilter {
    pub fn by_name<T: Into<String>>(name: T) -> Self {
        Self {
            name: Some(name.into()),
            id: None,
        }
    }

    pub fn by_id(id: Id) -> Self {
        Self {
            name: None,
            id: Some(id),
        }
    }
}

impl IntoQueryPairs for TeamFilter {
    fn into_query_pairs(self) -> Vec<(&'static str, String)> {
        let mut params = QueryParams::new();
        params.push_opt("name", self.name).push_opt(ID, self.id);

        params.build()
    }
}
