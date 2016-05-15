#[macro_export]
macro_rules! error {
    (
        $main_error_ident: ident {
            $($current_error_ident: ident),+
        }
    ) => {
        use std::error::Error as DefineErrorError;
        use std::fmt::Display as DefineErrorDisplay;
        use std::fmt::Formatter as DefineErrorFormatter;
        use std::fmt::Result as DefineErrorFmtResult;

        #[derive(Debug)]
        pub enum $main_error_ident {
            $($current_error_ident($current_error_ident)),+
        }

        impl DefineErrorDisplay for $main_error_ident {
            fn fmt(&self, f: &mut DefineErrorFormatter) -> DefineErrorFmtResult {
                match *self {
                    $($main_error_ident::$current_error_ident(ref err) => write!(f, "{}", err)),+
                }
            }
        }

        impl DefineErrorError for $main_error_ident {
            fn description(&self) -> &str {
                match *self {
                    $($main_error_ident::$current_error_ident(ref err) => err.description()),+
                }
            }

            fn cause(&self) -> Option<&DefineErrorError> {
                match *self {
                    $($main_error_ident::$current_error_ident(ref err) => Some(err)),+
                }
            }
        }

        $(
            impl From<$current_error_ident> for $main_error_ident {
                fn from(error: $current_error_ident) -> $main_error_ident {
                    $main_error_ident::$current_error_ident(error)
                }
            }
        )+
    }
}
