use std::collections::HashMap;

pub struct RequestMessage {
    method: Option<String>,
    url: Option<String>,
    version: Option<String>,
    header_lines: HashMap<Option<String>, Option<String>>,
    entity_body: Vec<u8>,
}

impl RequestMessage {
    pub fn new(buf: Vec<u8>) -> Option<RequestMessage> {
        let mut read_len = 0;
        let mut method = None;
        let mut url = None;
        let mut version = None;
        let mut header_lines = HashMap::new();
        let mut entity_body = Vec::new();
        let mut buf_iter = buf.iter();

        // request_line
        let pos = buf_iter.position(|chr| *chr == b'\n');
        if pos.is_none() {
            return None;
        }
        let request_line = String::from_utf8_lossy(&buf[0..=pos.unwrap()]).to_string();
        read_len += pos.unwrap() + 1;
        let mut request_line_iter = request_line
            .split_whitespace()
            .map(|elem| elem.trim().to_owned());
        method = request_line_iter.next();
        url = request_line_iter.next();
        version = request_line_iter.next();

        // header_lines
        loop {
            let pos = buf_iter.position(|chr| *chr == b'\n');
            if pos.is_none() {
                break;
            }
            let header_line = String::from_utf8_lossy(&buf[read_len..=(read_len + pos.unwrap())])
                .trim()
                .to_string();
            read_len += pos.unwrap() + 1;
            if header_line.is_empty() {
                break;
            }
            let mut header_line_iter = header_line.split(": ").map(|elem| elem.to_owned());
            header_lines.insert(header_line_iter.next(), header_line_iter.next());
        }
        println!("[INFO] Read: {}/{}", read_len, buf.len());

        // entity_body
        if read_len < buf.len() {
            let content_length = header_lines
                .get(&Some("Content-Length".to_owned()))
                .unwrap_or(&Some("0".to_owned()))
                .to_owned()
                .unwrap_or("0".to_owned())
                .trim()
                .parse::<usize>()
                .unwrap_or(0);
            println!("[INFO] Content Length: {}", content_length);
            entity_body = buf[read_len..(read_len + content_length)].to_owned();
        }

        Some(RequestMessage {
            method,
            url,
            version,
            header_lines,
            entity_body,
        })
    }

    pub fn get_url(&self) -> Option<String> {
        self.url.to_owned()
    }

    pub fn to_string(&self) -> String {
        let mut header_lines = String::new();
        for (header_field_name, value) in &self.header_lines {
            header_lines.push_str(
                format!(
                    "{}: {}\r\n",
                    header_field_name.to_owned().unwrap_or("".to_owned()),
                    value.to_owned().unwrap_or("".to_owned())
                )
                .as_str(),
            );
        }
        format!(
            "{} {} {}\r\n{}\r\n",
            self.method.to_owned().unwrap_or("".to_owned()),
            self.url.to_owned().unwrap_or("".to_owned()),
            self.version.to_owned().unwrap_or("".to_owned()),
            header_lines,
            /* String::from_utf8_lossy(&self.entity_body) */
        )
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut header_lines = String::new();
        for (header_field_name, value) in &self.header_lines {
            header_lines.push_str(
                format!(
                    "{}: {}\r\n",
                    header_field_name.to_owned().unwrap_or("".to_owned()),
                    value.to_owned().unwrap_or("".to_owned())
                )
                .as_str(),
            );
        }
        let mut vec = format!(
            "{} {} {}\r\n{}\r\n",
            self.method.to_owned().unwrap_or("".to_owned()),
            self.url.to_owned().unwrap_or("".to_owned()),
            self.version.to_owned().unwrap_or("".to_owned()),
            header_lines
        )
        .into_bytes();
        vec.extend(self.entity_body.iter());
        vec
    }
}

pub struct ResponseMessage {
    version: Option<String>,
    status_code: Option<String>,
    phrase: Option<String>,
    header_lines: HashMap<Option<String>, Option<String>>,
    entity_body: Vec<u8>,
}

impl ResponseMessage {
    pub fn new() -> ResponseMessage {
        ResponseMessage {
            version: None,
            status_code: None,
            phrase: None,
            header_lines: HashMap::new(),
            entity_body: Vec::new(),
        }
    }

    pub fn set_version(&mut self, version: &str) {
        self.version = Some(version.to_owned())
    }

    pub fn set_status_code(&mut self, status_code: &str) {
        self.status_code = Some(status_code.to_owned())
    }

    pub fn set_phrase(&mut self, phrase: &str) {
        self.phrase = Some(phrase.to_owned())
    }

    pub fn insert_header_line(&mut self, header_line_name: &str, value: &str) {
        self.header_lines
            .insert(Some(header_line_name.to_owned()), Some(value.to_owned()));
    }

    pub fn set_entity_body(&mut self, entity_body: Vec<u8>) {
        self.entity_body = entity_body;
    }

    pub fn to_string(&self) -> String {
        let mut header_lines = String::new();
        for (header_field_name, value) in &self.header_lines {
            header_lines.push_str(
                format!(
                    "{}: {}\r\n",
                    header_field_name.to_owned().unwrap_or("".to_owned()),
                    value.to_owned().unwrap_or("".to_owned())
                )
                .as_str(),
            );
        }
        format!(
            "{} {} {}\r\n{}\r\n",
            self.version.to_owned().unwrap_or("".to_owned()),
            self.status_code.to_owned().unwrap_or("".to_owned()),
            self.phrase.to_owned().unwrap_or("".to_owned()),
            header_lines,
            /* String::from_utf8_lossy(&self.entity_body) */
        )
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut header_lines = String::new();
        for (header_field_name, value) in &self.header_lines {
            header_lines.push_str(
                format!(
                    "{}: {}\r\n",
                    header_field_name.to_owned().unwrap_or("".to_owned()),
                    value.to_owned().unwrap_or("".to_owned())
                )
                .as_str(),
            );
        }
        let mut vec = format!(
            "{} {} {}\r\n{}\r\n",
            self.version.to_owned().unwrap_or("".to_owned()),
            self.status_code.to_owned().unwrap_or("".to_owned()),
            self.phrase.to_owned().unwrap_or("".to_owned()),
            header_lines
        )
        .into_bytes();
        vec.extend(self.entity_body.iter());
        vec
    }
}
