#[macro_export]
macro_rules! multi_line {
    ( $( $e:expr ),* $(,)? ) => {
        {
            let mut tmp: Vec<&str> = Vec::new();

            $(
                tmp.push($e);
            )*

            tmp.join("\n")
        }
    };
}

#[cfg(test)]
mod test {

    #[test]
    fn base() {

        let this = format!("{}", "this");

        let str = multi_line!(
            "test",
            &this,
            "",
            "works",
        );

        assert_eq!(str, "test\nthis\n\nworks");
    }

    #[test]
    fn empty() {
        let str1 = multi_line!("");
        let str2 = multi_line!("a");

        assert_eq!(str1, "");
        assert_eq!(str2, "a")
    }
}
