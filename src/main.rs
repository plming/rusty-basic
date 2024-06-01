fn main() {
    let sample_code = String::from(
        r#"
        PRINT "How many fibonacci numbers do you want?"
        INPUT nums
        
        LET a = 0
        LET b = 1
        WHILE nums > 0 REPEAT
            PRINT a
            LET c = a + b
            LET a = b
            LET b = c
            LET nums = nums - 1
        ENDWHILE
    "#);

    println!("List of tokens:");

    let tokens = tokenize(sample_code);
    println!("{tokens:?}");
}

pub fn tokenize(code: String) -> Vec<String> {
    unimplemented!();
}
