const DEFAULT_PATH: &str = "/";
const DEFAULT_HTTP_PORT: u16 = 80;
const DEFAULT_HTTPS_PORT: u16 = 443;

const PROTOCOL_HTTP: &str = "HTTP";
const PROTOCOL_HTTPS: &str = "HTTPS";

#[derive(Clone)]
pub struct ParsedURL {
    pub protocol: String,
    pub hostname: String,
    pub port: u16,
    pub path: String,
}

pub fn parse_url(url: &str) -> Result<ParsedURL, String> {
    let (protocol, hostportandpath) = url.split_once("://").unwrap_or((PROTOCOL_HTTP, url));
    let protocol = protocol.to_uppercase();

    let (hostport, path) = hostportandpath
        .split_once("/")
        .unwrap_or_else(|| (hostportandpath, ""));

    let (hostname, port) = match hostport.split_once(":") {
        Some((hostname, port)) => {
            let Ok(port) = port.parse::<u16>() else {
                return Err("The port is invalid".to_string());
            };
            (hostname, port)
        }
        None => match protocol.as_str() {
            PROTOCOL_HTTP => (hostport, DEFAULT_HTTP_PORT),
            PROTOCOL_HTTPS => (hostport, DEFAULT_HTTPS_PORT),
            _ => (hostport, DEFAULT_HTTP_PORT),
        },
    };

    let path = format!("{DEFAULT_PATH}{path}");

    return Ok(ParsedURL {
        protocol,
        hostname: hostname.to_string(),
        port,
        path,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_simple() {
        let url = "http://eu.httpbin.org/get";
        let ParsedURL {
            protocol,
            hostname,
            port,
            path,
        } = parse_url(url).unwrap();
        assert_eq!(protocol, PROTOCOL_HTTP);
        assert_eq!(hostname, "eu.httpbin.org".to_string());
        assert_eq!(port, 80);
        assert_eq!(path, "/get".to_string());
    }

    #[test]
    fn test_parsing_defaults() {
        let url = "http://eu.httpbin.org/";
        let ParsedURL {
            protocol,
            hostname,
            port,
            path,
        } = parse_url(url).unwrap();
        assert_eq!(protocol, PROTOCOL_HTTP);
        assert_eq!(hostname, "eu.httpbin.org".to_string());
        assert_eq!(port, DEFAULT_HTTP_PORT);
        assert_eq!(path, DEFAULT_PATH);
    }

    #[test]
    fn test_parsing_complex() {
        // TODO: update url with more fields
        let url = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";
        let ParsedURL {
            protocol,
            hostname,
            port,
            path,
        } = parse_url(url).unwrap();
        assert_eq!(protocol, PROTOCOL_HTTPS);
        assert_eq!(hostname, "github.com".to_string());
        assert_eq!(port, DEFAULT_HTTPS_PORT);
        assert_eq!(
            path,
            "/rust-lang/rust/issues?labels=E-easy&state=open".to_string()
        );
    }
}
