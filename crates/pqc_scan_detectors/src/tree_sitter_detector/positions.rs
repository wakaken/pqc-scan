//! Bounded decoded-source positions; no prefix rescans per AST candidate/hit.
pub(super) struct Positions {
    lines: Vec<usize>,
    wide: Vec<(usize, usize)>,
    covered: usize,
    pub complete: bool,
}
impl Positions {
    pub fn entries(&self) -> usize {
        self.lines.len() + self.wide.len()
    }
    pub fn new(text: &str, limit: usize) -> Self {
        let mut result = Self {
            lines: Vec::new(),
            wide: Vec::new(),
            covered: text.len(),
            complete: true,
        };
        if limit == 0 {
            result.covered = 0;
            result.complete = false;
            return result;
        }
        result.lines.push(0);
        let mut extra = 0;
        for (offset, c) in text.char_indices() {
            if c == '\n' || !c.is_ascii() {
                if result.lines.len() + result.wide.len() >= limit {
                    result.covered = offset;
                    result.complete = false;
                    break;
                }
                if c == '\n' {
                    result.lines.push(offset + 1);
                } else {
                    extra += c.len_utf8() - 1;
                    result.wide.push((offset + c.len_utf8(), extra));
                }
            }
        }
        result
    }
    pub fn at(&self, offset: usize) -> Option<(usize, usize)> {
        if offset > self.covered || self.lines.is_empty() {
            return None;
        }
        let line = self
            .lines
            .partition_point(|&start| start <= offset)
            .checked_sub(1)?;
        let start = self.lines[line];
        let extras = |end| {
            self.wide
                .partition_point(|&(pos, _)| pos <= end)
                .checked_sub(1)
                .map_or(0, |i| self.wide[i].1)
        };
        Some((
            line + 1,
            offset - start + 1 - (extras(offset) - extras(start)),
        ))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unicode_newlines_and_exact_entry_limits_match_original_coordinates() {
        let source = "α💻\nabc日本語\r\nlast";
        let p = Positions::new(source, 100);
        for offset in source.char_indices().map(|(n, _)| n).chain([source.len()]) {
            assert_eq!(
                p.at(offset),
                Some(pqc_scan_core::model::line_col_for_offset(source, offset))
            );
        }
        for (limit, complete) in [(0, false), (1, false), (2, false), (3, true), (4, true)] {
            let p = Positions::new("💻\nx", limit);
            assert_eq!(p.complete, complete, "{limit}");
            if complete {
                assert_eq!(p.at(5), Some((2, 1)));
            } else {
                assert!(p.at(6).is_none());
            }
        }
    }
}
