


fn main() {
    let mut memory: [u8; 5000] = [0;5000];
    let mut cursor: usize = 0;


    // #TODO implement repl
    //let bf_program = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]";
    let bf_program = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
                    //0 1 0
    for code in bf_program.chars() {
        match code {
            '+' => memory[cursor] += 1,
            '-' => if memory[cursor] > 0 {memory[cursor] -= 1},
            '>' => cursor += 1,
            '<' => if cursor > 0 { cursor -=1 }, 
            '.' => print!("{}", memory[cursor] as char),
            '[' => 
            {
                // #TODO refactor
                let mut precursor = memory[cursor] as i32;
                while precursor > 1 {
             
                    for option in bf_program[bf_program.find('[').unwrap() + 1..bf_program.find(']').unwrap() + 1].chars() {
                 

                        match option {
                            '+' => memory[cursor] +=1,
                            '-' => if memory[cursor] > 0 { memory[cursor] -= 1 },
                            '>' => cursor+=1,
                            '<' => cursor-=1,
                            ']' => cursor+=0,
                            _ => println!("{}", option),
                        };
                    } 
                    precursor -= 1;
                }

            }
            ']' => cursor +=0,
            _ => todo!("not implemented"),
        };
    }

   // println!("{:?}", &memory[0..4]);
    println!("{:?}", &memory[0..100]);
}
