/* For CGF : S->E+E/e 
     BY LEFT FACTORING BECAUSE OF THE NON DETERMINSTIC NATURE OF THE GRAMMAR 
     NEW CFG : 
            S->Es
            s->+Es/e 
*/

#[derive(Debug)]

struct Token
{ 
    input_string: String,
    parse_char: String,
    counter: usize
}

trait Copy
{ 
    fn copy(&self)->Self;
} 

impl Copy for Token 
{ 
    fn copy(&self)->Self {
        Token
        { 
            input_string: self.input_string.clone(),
            parse_char: self.parse_char.clone(),
            counter: self.counter
        } 
    }
}





fn S(token:Token)->Token
{ 
    let mut new_token = token.copy(); 
    if token.parse_char == "E"
    {
        println!("s->`Es");
        new_token = parse_match("E",token.copy()).unwrap();
        println!("S->E`s");
        new_token = s(new_token);
    }

    return new_token;

    
}

fn s(token:Token)->Token
{ 
    let mut new_token = token.copy();
    if token.parse_char == "+"
    { 
        println!("s->`+Es");
        new_token = parse_match("+",token.copy()).unwrap();
        println!("s->+`Es");
        new_token = parse_match("E",new_token).unwrap();
        println!("s->+E`s");
        return  s(new_token.copy());
    }
    
    if token.parse_char == "$"
    { 
        println!("s->`e");
    } 
    return new_token;
}

fn parse_match(char: &str,token:Token)->Result<Token,String>
{ 
    match char
    { 
        char if char == &token.parse_char => Ok(get_next_char(token)), 
        _ => Err("Error in parsing ".to_string()) 
    }
} 

fn get_next_char(token : Token)->Token
{
    let mut new_token  = token.copy(); 
    new_token.counter += 1;
    new_token.parse_char = token.input_string.chars().nth(new_token.counter).unwrap().to_string();
    return new_token; 
} 




fn main()
{ 
    let token = Token{ 
        input_string: "E+E+E$".to_string(),
        parse_char: "E".to_string(), 
        counter: 0
    }; 
    let new_token  = S(token);

    println!("{0:?}",new_token);

    if new_token.parse_char == "$"
    { 
        println!("Parsing Successful");
    }

    else 
    { 
        println!("Parsing Unsuccessful");
    }

    
}