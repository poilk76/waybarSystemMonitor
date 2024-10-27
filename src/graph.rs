pub fn create_graph(values:&Vec<u8>) -> String{
    let mut lines: Vec<String> = vec![String::from("100% │-"),String::from("80%  │-"),String::from("60%  │-"),String::from("40%  │-"),String::from("20%  │-")];
    let mut result  = String::from("");

    for value in values {
        for i in 0..5 {
            if *value > i * 20 {
                lines[(4-i) as usize].push('█');
            }
            else {
                lines[(4-i) as usize].push('-');
            }
        }
    }

    for line in lines {
        result += &line;
        result += "-│\\n";
    }

    return result
}

