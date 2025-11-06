#[derive(serde::Serialize, serde::Deserialize)]
pub struct ApiResult<T> {
    pub success: bool,
    pub message: Option<String>,
    pub payload: Option<T>,
}

impl<T> ApiResult<T> {
    pub fn success(payload: T, message: impl Into<Option<String>>) -> Self {
        Self {
            success: true,
            message: message.into(),
            payload: Some(payload),
        }
    }

    pub fn success_message(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
            payload: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: Some(message.into()),
            payload: None,
        }
    }
}

#[cfg_attr(test, derive(serde::Deserialize))]
#[derive(serde::Serialize)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: Option<u64>,
    pub next_page: Option<String>,
    pub prev_page: Option<String>,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub total_pages: Option<u64>,
}

impl<T> PaginatedResult<T> {
    pub fn new(
        items: Vec<T>,
        total: Option<u64>,
        page: Option<u64>,
        per_page: Option<u64>,
        total_pages: Option<u64>,
        next_page: Option<String>,
        prev_page: Option<String>,
    ) -> Self {
        Self {
            items,
            total,
            page,
            per_page,
            total_pages,
            next_page,
            prev_page,
        }
    }
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(serde::Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub after: Option<String>,
}
