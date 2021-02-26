use regex::Regex;

pub struct ResponseSearcher {
    line: String,
    index: usize,
}

impl ResponseSearcher {
    pub fn new(line: &str, index: usize) -> Self {
        ResponseSearcher {
            line: line.to_string(),
            index: index
        }
    }

    pub fn search(&self, search_text: &str) -> String {
        let re = Regex::new(search_text).unwrap();
        let m = re.captures(&self.line).unwrap();
        m[self.index].to_string()
    }

    pub fn exists(&self, search_text: &str) -> bool {
        let re = Regex::new(search_text).unwrap();
        re.is_match(&self.line)
    }
}