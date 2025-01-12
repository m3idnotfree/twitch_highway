macro_rules! request_struct {
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
        $($(#[$field_meta:meta])*
            $field_vis:vis $field:ident: $type:ty
        ),*$(,)?
        }
    ) => {
        $(#[$struct_meta])*
        pub struct $struct_name {
        $($(#[$field_meta])*
            $field_vis $field: Option<$type>
        ),*
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self::default()
            }

         $(pub fn $field(mut self, value: $type) -> Self
            {
                self.$field = Some(value);
                self
            })*
        }
    };
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
            required {
            $($(#[$req_meta:meta])*
            $fv:vis $req_field:ident: $req_type:ty ),*$(,)?
            }
        $(, optional {
            $($(#[$opt_meta:meta])*
              $ofv:vis $opt_field:ident: $opt_type:ty ),*$(,)?
            }
        )?
        }
    ) => {
        $(#[$struct_meta])*
        pub struct $struct_name {
            $($(#[$req_meta])*
               $fv $req_field: $req_type,
            )*
         $($($(#[$opt_meta])*
               $ofv $opt_field: Option<$opt_type>,
         )*)?
        }

        impl $struct_name {
            pub fn new(
            $($req_field: $req_type,)*
            ) -> Self {
                Self {
                $($req_field,)*
                $($($opt_field: None,)*)?
                }
            }

            $($(
                pub fn $opt_field(mut self, value: $opt_type) -> Self
                {
                    self.$opt_field = Some(value);
                    self
                }
            )*)?
        }
    };
}

macro_rules! new_request_struct {
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident {
    $(  string: {
            $($(#[$string_meta:meta])*
                $string_vis:vis $string_field:ident: $string_type:ty),*$(,)?
        }
    )?
    $(,
        any: {
            $($(#[$any_meta:meta])*
                $any_vis:vis $any_field:ident: $any_type:ty),*$(,)?
        }
    )?
    }
    ) => {
        $(#[$struct_meta])*
        pub struct $struct_name {
            $(
        $($(#[$string_meta])*
            $string_vis $string_field: Option<$string_type>,)*
            )?
        $($($(#[$any_meta])*
            $any_vis $any_field: Option<$any_type>,
        )*)?
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self::default()
            }

        $(
         $(pub fn $string_field<T: Into<String>>(mut self, value: T) -> Self
            {
                self.$string_field = Some(value.into());
                self
            }
         )*
        )?

        $(
         $(pub fn $any_field(mut self, value: $any_type) -> Self
            {
                self.$any_field = Some(value);
                self
            }
         )*
        )?
        }
    };
}
