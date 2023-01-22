use rs_web_response::*;

#[test]
fn test_web_error_struct() {
    let error = WebError {
        code: 500,
        description: "Internal Server Error".to_string(),
    };
    assert_eq!(error.code, 500);
    assert_eq!(error.description, "Internal Server Error".to_string());
}

#[test]
fn test_metadata_struct() {
    let meta: Metadata = Metadata {
        version: 1,
        data_type: "Metadata".to_string(),
    };
    assert_eq!(meta.version, 1);
    assert_eq!(meta.data_type, "Metadata".to_string());
}
