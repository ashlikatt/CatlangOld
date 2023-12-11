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

    match tokenize(string) {
        Ok(x) => {
            x.iter().for_each(|token| {
                println!("{:?}", token.token)
            });
        }
        Err(x) => {
            println!("ERROR: {}", x.get_message())
        }
    }

}

