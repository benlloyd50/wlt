#[derive(Default)]
pub struct Request {
    pub action: HttpAction,
    pub protocol: Protocol,
    pub path: String,
    pub hostname: String,
    pub port: u16,

    pub content_type: ConentType,
    pub content: String,
}

impl<'a> Request {
    pub fn to_string(&self) -> String {
        let mut request = String::new();

        // Request Headers
        request += &format!(
            "{} {} {}\n",
            self.action.to_string(),
            self.path,
            self.protocol.to_string(),
        );
        request += &format!("Host: {}\n", self.hostport());
        request +=
            "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:129.0) Gecko/20100101 Firefox/129.0\n";
        request += "Accept: text/html, application/json, application/xhtml+xml, application/xml;q=0.9, */*;q=0.8\n";
        request += "Accept-Language: en-US,en;q=0.5\n";
        request += "Accept-Encoding: gzip deflate\n";
        request += "Connection: close\n";
        request += "Upgrade-Insecure-Requests: 1\n";

        // Representation Headers
        request += &format!(
            "Content-Type: {}; charset=utf-8\n",
            self.content_type.to_string(),
        );
        request += &format!("Content-Length: {}\n", self.content_btyes_len());

        // Blank Line before content
        request += "\n";

        request += &format!("{}", self.content);

        request
    }

    fn content_btyes_len(&self) -> usize {
        self.content.as_bytes().len()
    }

    fn hostport(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}

#[derive(Debug)]
pub struct RequestBuilder<'a> {
    pub action: Option<&'a str>,
    pub protocol: Option<&'a str>,
    pub path: Option<&'a str>,
    pub hostname: Option<&'a str>,
    pub port: Option<u16>,
    pub content_type: Option<&'a str>,
}

// Lifetime: the str reference we passed in must live as long as the RequestBuilder is alive
#[allow(dead_code)]
impl<'a> RequestBuilder<'a> {
    pub fn empty() -> RequestBuilder<'a> {
        RequestBuilder {
            action: None,
            protocol: None,
            path: None,
            hostname: None,
            port: None,
            content_type: None,
        }
    }

    pub(crate) fn with_action(&'a mut self, action: &'a str) -> &'a mut Self {
        self.action = Some(action);
        self
    }

    pub(crate) fn with_protocol(&'a mut self, protocol: &'a str) -> &'a mut Self {
        self.protocol = Some(protocol);
        self
    }

    pub(crate) fn with_path(&'a mut self, path: &'a str) -> &'a mut Self {
        self.path = Some(path);
        self
    }

    pub(crate) fn with_hostname(&'a mut self, hostname: &'a str) -> &'a mut Self {
        self.hostname = Some(hostname);
        self
    }

    pub(crate) fn with_content_type(&'a mut self, content_type: &'a str) -> &'a mut Self {
        self.content_type = Some(content_type);
        self
    }

    pub(crate) fn with_port(&mut self, port: u16) -> &mut Self {
        self.port = Some(port);
        self
    }

    pub fn build(&mut self) -> Result<Request, RequestBuildError> {
        let mut req = Request::default();
        let mut error_messages = vec![];

        if let Some(action) = self.action {
            if let Some(action) = HttpAction::from_str(action) {
                req.action = action;
            } else {
                error_messages.push(format!("Action was invalid, {}", action));
            }
        }

        if let Some(protocol) = self.protocol {
            if let Some(protocol) = Protocol::from_str(protocol) {
                req.protocol = protocol;
            } else {
                error_messages.push(format!("Protocol was invalid, {}", protocol));
            }
        }

        if let Some(hostname) = self.hostname {
            req.hostname = hostname.to_string();
        } else {
            error_messages.push("No hostname defined in request.".to_string());
        }

        if let Some(port) = self.port {
            req.port = port;
        }

        if let Some(content_type) = self.content_type {
            if let Some(ct) = ConentType::from_str(content_type) {
                req.content_type = ct;
            } else {
                error_messages.push(format!("Content-type was invalid, {}", content_type));
            }
        }

        if error_messages.len() > 0 {
            return Err(RequestBuildError { error_messages });
        }

        Ok(req)
    }
}

#[derive(Debug)]
pub struct RequestBuildError {
    error_messages: Vec<String>,
}

#[derive(Default)]
pub enum ConentType {
    Json,
    Html,
    #[default]
    Plaintext,
}

impl ConentType {
    pub fn to_string(&self) -> String {
        match self {
            ConentType::Json => "application/json",
            ConentType::Html => "text/html",
            ConentType::Plaintext => "text/plaintext",
        }
        .to_string()
    }

    pub fn from_str(content_type: &str) -> Option<Self> {
        Some(match content_type {
            "application/json" => ConentType::Html,
            "text/html" => ConentType::Json,
            "text/plaintext" => ConentType::Plaintext,
            _ => return None,
        })
    }
}

#[derive(Default)]
pub enum HttpAction {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpAction {
    fn to_string(&self) -> String {
        match self {
            HttpAction::GET => "GET",
            HttpAction::POST => "POST",
            HttpAction::PUT => "PUT",
            HttpAction::DELETE => "DELETE",
        }
        .to_string()
    }

    fn from_str(action: &str) -> Option<Self> {
        Some(match action.to_uppercase().as_str() {
            "GET" => HttpAction::GET,
            "PUT" => HttpAction::PUT,
            "POST" => HttpAction::POST,
            "DELETE" => HttpAction::DELETE,
            _ => return None,
        })
    }
}

#[derive(Default)]
pub enum Protocol {
    #[default]
    HTTP1_1,
    HTTPS,
}

impl Protocol {
    fn to_string(&self) -> String {
        match self {
            Protocol::HTTP1_1 => "HTTP/1.1",
            Protocol::HTTPS => "HTTPS",
        }
        .to_string()
    }

    fn from_str(action: &str) -> Option<Self> {
        Some(match action.to_uppercase().as_str() {
            "HTTP" => Protocol::HTTP1_1,
            "HTTPS" => Protocol::HTTPS,
            _ => return None,
        })
    }
}
