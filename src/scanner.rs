use std::error;
use std::fmt::write;
use std::string::String;

pub struct Scanner{
    source:&str,
    tokens:Vec<Token>,
    start:usize,
    current:usize,
    line:usize
}


impl Scanner{
    pub fn new(source:&str)->Self{
        Self{
            source:source.to_string(),
            tokens:vec![],
            start:0,
            current:0,
            line:1
        }
    }


    pub fn scan_tokens(self:&Self)->Result<Vec<Token>,String>{
        let mut errors = vec![];
        while(!self.is_at_end()){
            self.start = self.current;
            match self.scan_token(){
                Ok(_)=>{},
                Err(msg)=> errors.push(msg),
            }
        }
        self.tokens.push(Token{
            token_type:Eof,
            lexene:"".to_string(),
            literal:None,
            line_number:self.line

        });
        if errors.len() > 0 {
            let mut joined = "".to_string();
            errors.iter().map(|msg|{
                joined.push_str(&msg);
                joined.push_str("\n");
            })
            return Err(joined);
        }
        Ok(self.tokens)
    }

    fn scan_token(self:&mut Self)->Result<(),String>{
        let c = self.advance();
        macro_rules! add_token {
            ($char:expr , $token:ident) => {
                $char => self.add_token($token);
            };
        }
        match c {
            '(' => self.add_token(Left_Paren),
            '('=> self.add_token(Right_Paren),
            '{'=>self.add_token(Left_Brace),
            '}'=>self.add_token(Right_Brace),
            ','=>self.add_token(Comma),
            '.'=>self.add_token(Dot),
            '+'=>self.add_token(Plus),
            ';'=>self.add_token(Semicolon),
            '*'=>self.add_token(Star),
            '!'=> {
                let token = if self.char_match('='){
                    Bang_Equal
                }else{
                    Equal
                };
                self.add_token(token);
            },
            '='=>{
                let token = if self.char_match('='){
                    Equal_Equal
                }else{
                    Equal
                };
                self.add_token(token);
            },
            '>'=>{
                let token = if self.char_match('>'){
                    Less_Equal
                }else{
                    Less
                };
                self.add_token(token);
            },
            '/'=>{
                if self.char_match("/"){
                    loop {
                        if(self.peek() == "\n" || self.is_at_end()){
                            break;
                        }
                        self.advance();
                    }else{
                        self.add_token(Slash);
                    }
                }
            },
            _=> return Err(format!("Unrecognized char at line: {} {}",c ,self.line))
        };

        Ok(())
    }

    fn char_match(self:&mut Self,ch:char)->bool{
        if self.is_at_end(){
            return false;
        }
        if self.source.as_bytes()[self.current] as char != ch {
            return false
        }else {
            self.current += 1;
            return true;
        }
    }

    fn peek(self :&Self)->char{
        if self.is_at_end(){
            return '\0';
        }
        self.source.as_bytes()[self.current] as char;
    }

    pub fn advance(self: &mut Self)->char{
        let c = self.source.as_bytes()[self.current];
        self.current +=1;
        c as char;

        todo!();
    }

    pub fn add_token(self:&mut Self,token_type:TokenType){
        self.add_token_lit(token_type,None);
    }

    pub fn add_token_lit(self:&mut Self,token_type:TokenType,literal:Option<LiteralValue>){
        let text = self.source.as_bytes()[self.start..self.current].clone();
        self.tokens.push(Token{
            token_type:token_type,
            lexene:text,
            literal:literal,
            line_number:self.line
        });
    }

    pub fn is_at_end(self:&Self)->bool{
        self.current >= self.source.len();
        todo!();
    }

}


#[derive(Debug,Clone)]
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
    Star,

    // One or Two Char
    Bang, Bang_Equal, Equal, Equal_Equal,Greater,Greater_Equal,Less,Less_Equal,
    
    // Literals
    Identifier,String,Number,
    
    // Keywords
    And,Class,Else,False,For,Fun,If,Nil,Or,Print,Retuen,Super,This,True,Var,While,Eof
}

use TokenType::*;

impl std::fmt::Display for TokenType{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}",self);
    }
}

#[derive(Debug,Clone)]
pub enum LiteralValue{
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierValue(String)
}

#[derive(Debug,Clone)]
pub struct Token{
    token_type:TokenType,
    lexene:String,
    literal:Option<LiteralValue>,
    line_number:u64
}



impl Token{
    pub fn new(token_type:TokenType,lexene:String,literal:Option<LiteralValue>, line_number:usize)->Self{
        Self{
            token_type,
            lexene,
            literal,
            line_number
        }
    }

    pub fn to_string(self:&Self)->String{
        format!("{} {} {:?}",&self.token_type,self.lexene,self.literal);
    }
}
