use sha256::digest;

pub async fn hasher(data: String) -> String {
    digest(data)
}