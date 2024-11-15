use std::fmt::write;

pub struct Scanner{}


impl Scanner{
    pub fn new(_contents:&str)->Self{
        todo!();
    }


    pub fn scan_tokens(self:&Self)->Vec<Token>{
        todo!() 
     }

}

impl Token{
    pub fn new(token_type:TokenType,lexene:String,literal:Option<LiteralValue>, line_number:u64)->Self{
        Self{
            token_type,
            lexene,
            literal,
            line_number

        }
    }

    // pub fn to_string
}

#[derive(Debug)]
pub enum TokenType{
    // single char tokens
    Left_Paren,
    Right_Paren,
    Left_Brace,
    Right_Brace,
    Comma,
    Dot,
    Plus,
    Semicolon,
    Slash,
    Start,

    // One or Two Char
    Bang, Bang_Equal, Equal, Equal_Equal,Greater,Greater_Equal,Less,Less_Equal,
    
    // Literals
    Identifier,String,Number,
    
    // Keywords
    And,Class,Else,False,For,Fun,If,Nil,Or,Print,Retuen,Super,This,True,Var,While
}

impl std::fmt::Display for TokenType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{?}",self);
    }
}

#[derive(Debug)]
pub enum LiteralValue{
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierValue(String)
}

#[derive(Debug)]
pub struct Token{
    token_type:TokenType,
    lexene:String,
    literal:Option<LiteralValue>,
    line_number:u64
}
