use crate::types::Id;

define_select!(
    #[derive(Debug)]
    TeamSelector<'a> {
        name: &'a str  as by_name,
        id: &'a Id => ID as by_id,
    };
    into_query
);
