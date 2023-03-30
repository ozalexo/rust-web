use rocket::http::{Header, ContentType};

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct AuthorizeErrorResponse {
    header: ContentType,
    message: String,
}

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct RefundErrorResponse {
    header: ContentType,
    message: String,
}

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct QueueErrorResponse {
    header: ContentType,
    message: String,
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct AuthorizeSuccessResponse {
    header: ContentType,
    message: String,
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct RefundSuccessResponse {
    header: ContentType,
    message: String,
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct QueueSuccessResponse {
    header: ContentType,
    message: String,
}
