#[macro_export]
macro_rules! include_bytes_platform {
    ($path:expr) => {{
        let path = if cfg!(windows) {
            $path.replace('/', "\\")
        } else {
            $path.replace('\\', "/")
        };
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", path))
    }};
}

// #[macro_export]
// macro_rules! include_str_platform {
//     ($path:expr) => {{
//         let path = if cfg!(windows) {
//             $path.replace('/', "\\")
//         } else {
//             $path.replace('\\', "/")
//         };
//         include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", path))
//     }};
// }
