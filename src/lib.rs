pub mod v1 {
    #[derive(Debug)]
    pub struct WebError {
        pub code: i16,
        pub description: String,
    }

    #[derive(Debug)]

    pub struct Metadata {
        pub version: i16,
        pub data_type: String,
    }

    #[derive(Debug)]
    pub struct WebResponse<T> {
        pub success: bool,
        pub metadata: Metadata,
        pub error: Option<WebError>,
        pub data: Option<Box<T>>,
    }
}
