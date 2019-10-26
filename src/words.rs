pub struct Words<'a> {
    data: &'a str,
    position: usize,
}

impl<'a> Words<'a> {
    pub fn new(data: &'a str) -> Words<'a> {
        Words {
            data: data,
            position: 0,
        }
    }
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let mut start = self.position;
        let mut end = self.position;

        for (size, c) in self.data[self.position..]
            .chars()
            .map(|c| (c.len_utf8(), c))
        {
            match c {
                ':' | ',' | '.' | '?' | '!' | '(' | ')' | '\n' => {
                    if start != end {
                        return Some(&self.data[start..end]);
                    }

                    self.position += size;
                    return Some(&self.data[start..end + size]);
                }
                ' ' => {
                    if start != end {
                        return Some(&self.data[start..end]);
                    }

                    start += size;
                    end += size;
                    self.position += size;
                }
                _ => {
                    end += size;
                    self.position += size;
                }
            }
        }

        if start == end {
            return None;
        }

        Some(&self.data[start..end])
    }
}
