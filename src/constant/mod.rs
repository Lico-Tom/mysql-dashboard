use lazy_static::lazy_static;

lazy_static! {
    pub static ref MYSQL_HOST: String = std::env::var("MYSQL_HOST").unwrap_or_else(|_| "localhost".to_string());
    pub static ref MYSQL_PORT: u16 = std::env::var("MYSQL_PORT").unwrap_or_else(|_| "3306".to_string()).parse().unwrap();
    pub static ref MYSQL_USERNAME: String = std::env::var("MYSQL_USERNAME").unwrap_or_else(|_| "hzj".to_string());
    pub static ref MYSQL_PASSWORD: String = std::env::var("MYSQL_PASSWORD").unwrap_or_else(|_| "Mysql@123".to_string());
    pub static ref MYSQL_URL: String = format!("mysql://{}:{}@{}:{}/", MYSQL_USERNAME.as_str(), MYSQL_PASSWORD.as_str(), MYSQL_HOST.as_str(), MYSQL_PORT.to_owned());
}
