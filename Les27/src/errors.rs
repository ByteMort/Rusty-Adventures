use axum::{Json, extract::rejection::JsonRejection, http::StatusCode, response::{IntoResponse, Response}};

pub struct CustomJsonError(pub JsonRejection);

impl IntoResponse for CustomJsonError{    
    fn into_response(self) -> Response {
        let (status, message) = match self.0 {
            JsonRejection::JsonSyntaxError(_) => {
                (StatusCode::BAD_REQUEST, "An invalid or empty JSON body was sent.".to_owned())
            }
            JsonRejection::JsonDataError(err) => {
                let err_msg = err.to_string();
                find_missing_part(err_msg)
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "A JSON error occurred on the server.".to_owned())
        };

        (status, Json(serde_json::json!({"message": message}))).into_response()
    }
}

fn find_missing_part(msg: String) -> (StatusCode, String){
    if let Some(start) = msg.find("missing field `"){
        let field_start = start + "missing field `".len();

        if let Some(end) = msg[field_start..].find("`"){
            let field_name = &msg[field_start..field_start+end];
            
            let dynamic_msg = format!("Please fill in the required '{}' field", field_name);

            return (StatusCode::BAD_REQUEST, dynamic_msg);
        }
    }
    return (StatusCode::BAD_REQUEST, "The specified data format is invalid.".to_string());
}