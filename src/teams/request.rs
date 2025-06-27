use crate::types::Id;

define_select!(
    #[derive(Debug)]
    TeamSelector {
        name: String as by_name,
        id: Id => ID as by_id,
    };
    apply_to_url
);
