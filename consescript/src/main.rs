mod lib;
use lib::conselexer;
use lib::consesyntax;
use std::io;
use std::collections::HashMap;

fn main() {
    let mut input_str = String::new();

    let mut varMap = HashMap::new();
    while true {
        input_str = String::new();
        match io::stdin().read_line(&mut input_str) {
            Ok(n) => {
                if input_str.as_str() == "q" || input_str.as_str() == "quit" {
                    break;
                }

                //input_str.replace("\n")
                let mut tokens = conselexer::lexerParse(input_str.as_str());


                let ast_node = consesyntax::syntaxParse(&mut tokens);

                let mut node = ast_node.unwrap();

                consesyntax::executeSripte(&mut node, &mut varMap);
            }
            Err(error) => panic!("error: {}", error),
        }
    }



}
