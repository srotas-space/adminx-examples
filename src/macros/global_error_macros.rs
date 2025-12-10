// src/macros/global_error_macros.rs

#[macro_export]
macro_rules! handle_global_error {
    (bad_request) => {{
        return Err($crate::errors::global_error::GlobalError::bad_request(None, None).into());
    }};
    (bad_request, $code:expr) => {{
        return Err($crate::errors::global_error::GlobalError::bad_request(Some($code), None).into());
    }};
    (bad_request, $code:expr, $msg:expr) => {{
        return Err($crate::errors::global_error::GlobalError::bad_request(Some($code), Some($msg)).into());
    }};

    (invalid_request) => {{
        return Err($crate::errors::global_error::GlobalError::invalid_request(None, None).into());
    }};
    (invalid_request, $code:expr) => {{
        return Err($crate::errors::global_error::GlobalError::invalid_request(Some($code), None).into());
    }};
    (invalid_request, $code:expr, $msg:expr) => {{
        return Err($crate::errors::global_error::GlobalError::invalid_request(Some($code), Some($msg)).into());
    }};

    (internal_error) => {{
        return Err($crate::errors::global_error::GlobalError::internal_error(None, None).into());
    }};
    (internal_error, $code:expr) => {{
        return Err($crate::errors::global_error::GlobalError::internal_error(Some($code), None).into());
    }};
    (internal_error, $code:expr, $msg:expr) => {{
        return Err($crate::errors::global_error::GlobalError::internal_error(Some($code), Some($msg)).into());
    }};

    (unauthorized_error) => {{
        return Err($crate::errors::global_error::GlobalError::unauthorized_error(None, None).into());
    }};
    (unauthorized_error, $code:expr) => {{
        return Err($crate::errors::global_error::GlobalError::unauthorized_error(Some($code), None).into());
    }};
    (unauthorized_error, $code:expr, $msg:expr) => {{
        return Err($crate::errors::global_error::GlobalError::unauthorized_error(Some($code), Some($msg)).into());
    }};

    (not_found_error) => {{
        return Err($crate::errors::global_error::GlobalError::not_found_error(None, None).into());
    }};
    (not_found_error, $code:expr) => {{
        return Err($crate::errors::global_error::GlobalError::not_found_error(Some($code), None).into());
    }};
    (not_found_error, $code:expr, $msg:expr) => {{
        return Err($crate::errors::global_error::GlobalError::not_found_error(Some($code), Some($msg)).into());
    }};


    (conflict_request) => {{
        return Err($crate::errors::global_error::GlobalError::conflict_request(None, None).into());
    }};
    (conflict_request, $code:expr) => {{
        return Err($crate::errors::global_error::GlobalError::conflict_request(Some($code), None).into());
    }};
    (conflict_request, $code:expr, $msg:expr) => {{
        return Err($crate::errors::global_error::GlobalError::conflict_request(Some($code), Some($msg)).into());
    }};


    // fallback
    ($variant:ident, $($rest:tt)*) => {
        compile_error!(concat!("Unknown variant: ", stringify!($variant), ". Use one of: bad_request, invalid_request, internal_error."));
    };
}
