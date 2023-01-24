use rs_web_response::v1::*;

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

#[test]
fn test_web_response_struct_string_success() {
    let response: WebResponse<String> = WebResponse::<String> {
        success: true,
        metadata: Metadata {
            version: 1,
            data_type: "Success".to_string(),
        },
        error: None,
        data: Some(Box::new(String::from("OK"))),
    };

    assert_eq!(response.success, true);
    assert_eq!(response.metadata.version, 1);
    assert_eq!(response.metadata.data_type, "Success".to_string());
    assert!(response.error.is_none());

    let value: Option<Box<String>> = response.data;

    assert_eq!(
        value.unwrap_or_else(|| Box::new(String::from(""))),
        Box::new(String::from("OK"))
    );
}
