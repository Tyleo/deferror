#[macro_export]
macro_rules! error {
    (
        $(#[$main_attribute: meta])*
        mainerror $main_error_ident: ident {
            $(
                $(#[$current_attribute: meta])*
                suberror $current_error_ident: ident
            ),*
        }
    ) => {
        $(#[$main_attribute])*
        pub enum $main_error_ident {
            $(
                $(#[$current_attribute])*
                $current_error_ident($current_error_ident)
            ),*
        }

        impl Display for $main_error_ident {
            fn fmt(&self, _f: &mut Formatter) -> Result {
                match *self {
                    $($main_error_ident::$current_error_ident(ref err) => write!(_f, "{}", err)),*
                }
            }
        }

        impl Error for $main_error_ident {
            fn description(&self) -> &str {
                match *self {
                    $($main_error_ident::$current_error_ident(ref err) => err.description()),*
                }
            }

            fn cause(&self) -> Option<&Error> {
                match *self {
                    $($main_error_ident::$current_error_ident(ref err) => Some(err)),*
                }
            }
        }

        $(
            impl From<$current_error_ident> for $main_error_ident {
                fn from(error: $current_error_ident) -> $main_error_ident {
                    $main_error_ident::$current_error_ident(error)
                }
            }
        )*
    }
}
