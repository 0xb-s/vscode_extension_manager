pub struct NetworkUtils;

impl NetworkUtils {
    pub fn validate_url(url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }

    pub fn encode_query_params(params: &[(String, String)]) -> String {
        let mut query = String::new();
        for (key, value) in params {
            if !query.is_empty() {
                query.push('&');
            }
            query.push_str(&format!("{}={}", key, value));
        }
        query
    }

    pub fn decode_query_params(query: &str) -> Vec<(String, String)> {
        query
            .split('&')
            .filter_map(|pair| {
                let mut iter = pair.splitn(2, '=');
                if let (Some(key), Some(value)) = (iter.next(), iter.next()) {
                    Some((key.to_string(), value.to_string()))
                } else {
                    None
                }
            })
            .collect()
    }
}
