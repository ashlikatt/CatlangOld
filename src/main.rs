use crate::tokenization::tokenize;

mod tokenization;

fn main() {
    let string = r#"
        2 3.0 .4 0.4 2.
        if (x == y) {

        }

        fn h(a, b) {
          return 2;
        }
    "#;

    let token_output = tokenize(string);

    let Ok(tokens) = token_output else {
        println!("ERROR: {}", token_output.err().unwrap().get_message());
        return;
    };

    tokens.iter().for_each(|token| {
        println!("{:?}", token.token)
    });
}

