pub mod lexer {
    pub fn lexer_main
    (
    asembly_input: &str
    ) -> Vec<String> {
        let mut comment: bool = false;
        let mut tokens: Vec<String> = Vec::new();
        let mut token: String = String::new();
        for i in 0..asembly_input.len() {
            let character = asembly_input.chars().nth(i).unwrap();
            match character{
                ';' => {comment = true}
                '\n' => {
                    if !comment && token !=  "" {
                        tokens.push(token);
                        token = String::new();
                    }
                    else {
                        comment = false;
                    }
                }
                ' ' => {
                    if !comment && token != ""{
                        tokens.push(token);
                        token = String::new();
                    }
                    
                }
                ',' => {}
                _ => {
                    if !comment {
                        String::push(&mut token, character) 
                    }
                    
                }
                
            }
            dbg!(&token);
        }
        if token != ""{
        tokens.push(token);
        };
        tokens
    }
}
