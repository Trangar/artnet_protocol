macro_rules! data_structure {
    (
        $(#[$outer:meta])*
        pub struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                pub $field:ident : $ty:ty,
            )*
        }
    ) => {
        $(#[$outer])*
        pub struct $name {
            $(
                $(#[$field_meta])*
                pub $field: $ty,
            )*
        }

        impl $name {
            /// Convert this struct to a byte array.
            pub fn to_bytes(&self) -> ::Result<Vec<u8>> {
                use convert::Convertable;
                use failure::ResultExt;

                let mut result = Vec::new();
                $(
                    self.$field.into_buffer(&mut result)
                        .context(concat!("Could not serialize field ", stringify!($name), "::", stringify!($field)))?;
                )*;
                Ok(result)
            }

            /// Convert a byte array to an instance of this struct.
            pub fn from(data: &[u8]) -> ::Result<$name> {
                use convert::Convertable;
                use failure::ResultExt;

                let mut cursor = ::std::io::Cursor::new(data);
                $(
                    let $field: $ty = Convertable::from_cursor(&mut cursor)
                        .context(concat!("Could not deserialize field ", stringify!($name), "::", stringify!($field)))?;
                )*
                Ok($name {
                    $($field, )*
                })
            }
        }


        #[test]
        fn test_encode_decode() {
            let start = $name {
                $(
                    $field: ::convert::Convertable::get_test_value(),
                )*
            };
            let bytes = start.to_bytes().expect("Could not serialize");
            let end = $name::from(&bytes).expect("Could not deserialize");
            $(
                assert!(::convert::Convertable::is_equal(&start.$field, &end.$field));
            )*
        }
    };
}
