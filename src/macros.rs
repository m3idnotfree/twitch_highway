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

         $(pub fn $field<T>(mut self, value: T) -> Self
            where
                T: Into<$type>
            {
                self.$field = Some(value.into());
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
                pub fn $opt_field<T>(mut self, value: T) -> Self
                where
                    T: Into<$opt_type>
                {
                    self.$opt_field = Some(value.into());
                    self
                }
            )*)?
        }
    };
}
