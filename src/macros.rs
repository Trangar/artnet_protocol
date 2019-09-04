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
            pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
                use crate::convert::Convertable;
                use crate::Error;

                let mut result = Vec::new();
                $(
                    self.$field.into_buffer(&mut result)
                        .map_err(|e| Error::SerializeError(concat!("Could not serialize field ", stringify!($name), "::", stringify!($field)), Box::new(e)))?;
                )*
                Ok(result)
            }

            /// Convert a byte array to an instance of this struct.
            pub fn from(data: &[u8]) -> crate::Result<$name> {
                use crate::convert::Convertable;
                use crate::Error;

                let mut cursor = ::std::io::Cursor::new(data);
                $(
                    let $field: $ty = Convertable::from_cursor(&mut cursor)
                        .map_err(|e| Error::DeserializeError(concat!("Could not deserialize field ", stringify!($name), "::", stringify!($field)), Box::new(e)))?;
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
