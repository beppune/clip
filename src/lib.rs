
fn parse(line: &str) -> usize {
    line.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {

        let line: &'static str = "-f -grp arg pos1 --longflag --longopt arg pos2";

        assert!( 46 == parse(&line) );  

    }

}
