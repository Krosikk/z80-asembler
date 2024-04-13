pub mod parser {
    pub fn parser_main(
        tokens: Vec<String>,
    ) -> Vec<u8> {
        let mut program_counter: u128 = 0;
        let mut inc: usize = 0;
        let mut labels: Vec<(String, u128)> = Vec::new();
        if tokens[0] == "org"{
            program_counter = tokens[1].parse().unwrap();
            dbg!(&program_counter);
            inc = 2;
        }
        
        dbg!(&inc);
        for i in inc..tokens.len(){
            dbg!(&tokens[i]);
            if tokens[i].chars().last().unwrap() == ':'{
                labels.push((tokens[i].clone()[0..tokens[i].len()-1].to_string(), program_counter));
                continue;
            }
            program_counter+=1;
        };
        for i in inc..tokens.len(){
            match tokens[i] {
                
            }
        }
        dbg!(labels);
        let xd: Vec<u8> = Vec::new(); 
        xd
    }
}