use std::collections::HashMap;
use std::path::{PathBuf, Path};
use serde::{Deserialize, Serialize};
pub type DocFreq = HashMap<String, usize>;
pub type TermFreq = HashMap<String, usize>;
pub type TermFreqPerDoc = HashMap<PathBuf, TermFreq>;

#[derive(Default, Deserialize, Serialize)]
pub struct Model {
    pub tfpd: TermFreqPerDoc, // how many time this term appear
    pub df: DocFreq, // how many document this term exist in 
}

// apply the term frequency formula
pub fn compute_tf(t: &str, d: &TermFreq) -> f32 {
    let a = d.get(t).cloned().unwrap_or(0) as f32;
    let b = d.iter().map(|(_, f)| *f).sum::<usize>() as f32;
    a / b
}

pub fn compute_idf(t: &str, n:usize, df: &DocFreq) -> f32 {
    let N = n as f32;
    let M = df.get(t).cloned().unwrap_or(1) as f32;
    return (N / M).log10();
}


pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }
    
    fn trim_left(&mut self) {
        // trim whitespaes from the left
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..]
        } 
    }

    fn chop(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }
    
    fn chop_while<P>(&mut self, mut predicate: P) ->  &'a[char] where P: FnMut(&char) -> bool {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        self.chop(n)

    }

    fn next_token(&mut self) -> Option<String> {
        self.trim_left();
        if self.content.len() == 0 {
            return None
        }
        
        if self.content[0].is_numeric() {
            return Some(self.chop_while(|x| x.is_numeric()).iter().collect());
        }

        if self.content[0].is_alphabetic() {
            return Some(self.chop_while(|x| x.is_alphanumeric()).iter().map(|x| x.to_ascii_uppercase()).collect());
        }
        return Some(self.chop(1).iter().collect());
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}


pub fn search_query<'a>(model: &'a Model, query: &'a [char]) -> Vec<(&'a Path, f32)> {
    let mut result = Vec::<(&Path, f32)>::new();
    let tokens = Lexer::new(&query).collect::<Vec<_>>();
    for (path, tf_table) in &model.tfpd {
        let mut rank = 0 as f32;
        for token in &tokens {
            rank += compute_tf(&token, &tf_table) * compute_idf(&token, model.tfpd.len(), &model.df);
        }
        result.push((path, rank));
    }
    result.sort_by(|(_, rank1), (_, rank2)| rank2.partial_cmp(rank1).unwrap());
    result
}
