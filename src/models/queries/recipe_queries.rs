use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchInfo {
    pub limit: i64,
    pub order_by: String,
    pub filter_by: Option<String>,
    pub search_term: Option<String>,
}

impl Default for SearchInfo {
    fn default() -> Self {
        SearchInfo {
            limit: 50,
            order_by: "created_at".to_string(),
            filter_by: None,
            search_term: None,
        }
    }
}
