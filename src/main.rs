mod lexer;

fn main() {
    let complex_sample_code = String::from(
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
    "#,
    );

    let sample_code = String::from("PRINT 2+3");

    println!("List of tokens:");

    let tokens = lexer::lex(&sample_code);

    println!("{tokens:?}")
}
