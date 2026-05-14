#[cfg(test)]
mod tests {
    use crate::logger::entry::{HttpMethod, LogEntry};
    use crate::logger::store::LogStore;

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry::new(1, HttpMethod::GET, "http://example.com".to_string());
        assert_eq!(entry.id, 1);
        assert_eq!(entry.method, HttpMethod::GET);
        assert_eq!(entry.url, "http://example.com");
        assert!(entry.status_code.is_none());
        assert!(entry.request_body.is_none());
        assert!(entry.response_body.is_none());
    }

    #[test]
    fn test_log_entry_set_response() {
        let mut entry = LogEntry::new(1, HttpMethod::POST, "http://example.com/api".to_string());
        let headers = vec![("content-type".to_string(), "application/json".to_string())];
        entry.set_response(200, headers.clone(), Some(b"{\"ok\": true}".to_vec()));
        assert_eq!(entry.status_code, Some(200));
        assert_eq!(entry.response_headers, headers);
        assert!(entry.response_body.is_some());
    }

    #[test]
    fn test_log_store_insert_and_retrieve() {
        let store = LogStore::new();
        let id = store.next_id();
        let entry = LogEntry::new(id, HttpMethod::GET, "http://example.com".to_string());
        store.insert(entry);
        assert_eq!(store.len(), 1);
        let retrieved = store.get_by_id(id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().url, "http://example.com");
    }

    #[test]
    fn test_log_store_update() {
        let store = LogStore::new();
        let id = store.next_id();
        let entry = LogEntry::new(id, HttpMethod::GET, "http://example.com".to_string());
        store.insert(entry);
        store.update(id, |e| e.set_response(404, vec![], None));
        let updated = store.get_by_id(id).unwrap();
        assert_eq!(updated.status_code, Some(404));
    }

    #[test]
    fn test_log_store_clear() {
        let store = LogStore::new();
        for i in 1..=5 {
            let entry = LogEntry::new(i, HttpMethod::GET, format!("http://example.com/{}", i));
            store.insert(entry);
        }
        assert_eq!(store.len(), 5);
        store.clear();
        assert!(store.is_empty());
    }

    #[test]
    fn test_http_method_display() {
        assert_eq!(HttpMethod::GET.to_string(), "GET");
        assert_eq!(HttpMethod::POST.to_string(), "POST");
        assert_eq!(HttpMethod::Other("CONNECT".to_string()).to_string(), "CONNECT");
    }
}
