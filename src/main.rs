/*Rust parser
Programmed by Cypress Payne
For CSC 3310
*/

use std::env;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

//main function
fn main() -> std::io::Result<()>{
    //get parameters and store in args vector
    let args: Vec<String> = env::args().collect();
    if args.len() != 3{
        panic!("Incorrect number of params");
    }
    //if correct number of parameters, run intended program
    if args.len() == 3{
        //get filename and open it, put file contents into string
        let filename = &args[1];
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        //get tag parameter
        let tag = &args[2];
        //if tag is not for scheme or prolog end program
        if tag != "-s" && tag != "-p"{
            panic!("Unrecognized tag");
        }

        //process file input, ensure it is in language, output processing info in scheme or prolog
        if tag == "-s"{
            println!("; processing input file {}", filename);
        }
        if tag == "-p"{
            println!("/* processing input file {}", filename);
        }

        //run file through lexer
        //first tokenize contents string
        contents = tokenizer(&contents);
        //next seperate tokens into a vector
        let tokens: Vec<&str>= contents.split("\n").collect();
        //test tokens to ensure they are valid
        for x in 0..tokens.len()-2{
            let test = test_tokens(x, &tokens);
            //if not valid tokens, file failes lexical analysis
            if test == false{
                panic!("Lexical analysis failed");
            }
        }

        //Parser - check that grammar is correct by comparing tokenized string to a regex experession
        let myregex = r"(ID [a-z]+\nASSIGN\nPOINT\nLPAREN\nNUM \d+\nCOMMA\nNUM \d+\nRPAREN\nSEMICOLON\n){2}(ID [a-z]+\nASSIGN\nPOINT\nLPAREN\nNUM \d+\nCOMMA\nNUM \d+\nRPAREN\nPERIOD\n*)";
        let re = Regex::new(myregex).unwrap();

        //if it is, file passes both lexical and syntax analysis, output message in scheme or prolog
        if re.is_match(&contents) {
            if tag == "-s"{
                println!("; Lexical and Syntax analysis passed");
            }
            if tag == "-p"{
                println!("   Lexical and Syntax analysis passed */");
            }
        }
        //else, error, end program
        else {
            panic!("Syntax analysis failed");
        }

        //continue if file passes
        //collect numbers from file for generating code
        let v: Vec<&str> = contents.matches(char::is_numeric).collect();

        //if scheme tag
        if tag == "-s"{
        //generate scheme code
            let mut code = String::new();
            code.push_str("(calculate-triangle (make-point ");
            code.push_str(v[0]);
            code.push_str(" ");
            code.push_str(v[1]);
            code.push_str(") (make-point ");
            code.push_str(v[2]);
            code.push_str(" ");
            code.push_str(v[3]);
            code.push_str(") (make-point ");
            code.push_str(v[4]);
            code.push_str(" ");
            code.push_str(v[5]);
            code.push_str("))");
            println!("{}", &code);
        }

        //if prolog tag
        if tag == "-p"{
        //generate prolog code
            let mut code = String::new();
            //line
            code.push_str("query(line(point2d("); code.push_str(v[0]); code.push_str(",");
            code.push_str(v[1]); code.push_str("), point2d("); code.push_str(v[2]);
            code.push_str(","); code.push_str(v[3]); code.push_str("), point2d(");
            code.push_str(v[4]); code.push_str(","); code.push_str(v[5]); code.push_str(")))\n");
            //triangle
            code.push_str("query(triangle(point2d("); code.push_str(v[0]); code.push_str(",");
            code.push_str(v[1]); code.push_str("), point2d("); code.push_str(v[2]);
            code.push_str(","); code.push_str(v[3]); code.push_str("), point2d(");
            code.push_str(v[4]); code.push_str(","); code.push_str(v[5]); code.push_str(")))\n");
            //vertical
            code.push_str("query(vertical(point2d("); code.push_str(v[0]); code.push_str(",");
            code.push_str(v[1]); code.push_str("), point2d("); code.push_str(v[2]);
            code.push_str(","); code.push_str(v[3]); code.push_str("), point2d("); code.push_str(v[4]);
            code.push_str(","); code.push_str(v[5]); code.push_str(")))\n");
            //horizontal
            code.push_str("query(horizontal(point2d("); code.push_str(v[0]); code.push_str(",");
            code.push_str(v[1]); code.push_str("), point2d("); code.push_str(v[2]);
            code.push_str(","); code.push_str(v[3]); code.push_str("), point2d(");
            code.push_str(v[4]); code.push_str(","); code.push_str(v[5]); code.push_str(")))\n");
            //equilateral
            code.push_str("query(equilateral(point2d("); code.push_str(v[0]); code.push_str(",");
            code.push_str(v[1]); code.push_str("), point2d("); code.push_str(v[2]);
            code.push_str(","); code.push_str(v[3]); code.push_str("), point2d(");
            code.push_str(v[4]); code.push_str(","); code.push_str(v[5]); code.push_str(")))\n");
            //isosceles
            code.push_str("query(isosceles(point2d("); code.push_str(v[0]); code.push_str(",");
            code.push_str(v[1]); code.push_str("), point2d("); code.push_str(v[2]);
            code.push_str(","); code.push_str(v[3]); code.push_str("), point2d(");
            code.push_str(v[4]); code.push_str(","); code.push_str(v[5]); code.push_str(")))\n");
            //right
            code.push_str("query(right(point2d("); code.push_str(v[0]); code.push_str(",");
            code.push_str(v[1]); code.push_str("), point2d("); code.push_str(v[2]);
            code.push_str(","); code.push_str(v[3]); code.push_str("), point2d(");
            code.push_str(v[4]); code.push_str(","); code.push_str(v[5]); code.push_str(")))\n");
            //scalene
            code.push_str("query(scalene(point2d("); code.push_str(v[0]); code.push_str(",");
            code.push_str(v[1]); code.push_str("), point2d("); code.push_str(v[2]);
            code.push_str(","); code.push_str(v[3]); code.push_str("), point2d(");
            code.push_str(v[4]); code.push_str(","); code.push_str(v[5]); code.push_str(")))\n");
            //acute
            code.push_str("query(acute(point2d("); code.push_str(v[0]); code.push_str(",");
            code.push_str(v[1]); code.push_str("), point2d("); code.push_str(v[2]);
            code.push_str(","); code.push_str(v[3]); code.push_str("), point2d(");
            code.push_str(v[4]); code.push_str(","); code.push_str(v[5]); code.push_str(")))\n");
            //obtuse
            code.push_str("query(obtuse(point2d("); code.push_str(v[0]); code.push_str(","); code.push_str(v[1]);
            code.push_str("), point2d("); code.push_str(v[2]); code.push_str(",");
            code.push_str(v[3]); code.push_str("), point2d("); code.push_str(v[4]);
            code.push_str(","); code.push_str(v[5]); code.push_str(")))\n");
            //finishing up
            code.push_str("writeln(T) :- write(T), nl.\nmain:- forall(query(Q), Q-> (writeln(‘yes’)) ; (writeln(‘no’))),\n      halt.");
            println!("{}", &code);
        }
        //extra credit create tree
        create_tree( &filename, &tokens).expect("Failed to create parse tree");
    }
    Ok(())
}

//tokenizer function
/*this function takes the string produced from the file
and swaps out recognized symbols with their designated tokens
as given in the language definition. It then returns the tokenized
string to be used in lexical analysis */
fn tokenizer(contents: &String) -> String{
    let re = Regex::new(r"\s").unwrap();
    let contents = re.replace_all(&contents, "");
    let re = Regex::new(r"point").unwrap();
    let contents = re.replace_all(&contents, "POINT\n");
    let re = Regex::new(r";").unwrap();
    let contents = re.replace_all(&contents, "SEMICOLON\n");
    let re = Regex::new(r",").unwrap();
    let contents = re.replace_all(&contents, "COMMA\n");
    let re = Regex::new(r"\.").unwrap();
    let contents = re.replace_all(&contents, "PERIOD\n");
    let re = Regex::new(r"\(").unwrap();
    let contents = re.replace_all(&contents, "LPAREN\n");
    let re = Regex::new(r"\)").unwrap();
    let contents = re.replace_all(&contents, "RPAREN\n");
    let re = Regex::new(r"=").unwrap();
    let contents = re.replace_all(&contents, "ASSIGN\n");
    let re = Regex::new(r"(?P<num>\d+)").unwrap();
    let contents = re.replace_all(&contents, "NUM $num\n");
    let re = Regex::new(r"(?P<id>[a-z]+)").unwrap();
    let contents = re.replace_all(&contents, "ID $id\n");
    return contents.to_string();
}

//test_tokens function
/*This function takes a vector containing assumed tokens that have been
created by the tokenizer and tests them to ensure that there are no
unaccounted for symbols. If there are, it will return false and the
lexical analyzer will determine that the file did not pass*/
fn test_tokens(x: usize, tokens: &Vec<&str>) -> bool{
    if tokens[x] == "POINT"{
        return true;
    }
    if tokens[x] == "SEMICOLON"{
        return true;
    }
    if tokens[x] == "COMMA"{
        return true;
    }
    if tokens[x] == "PERIOD"{
        return true;
    }
    if tokens[x] == "LPAREN"{
        return true;
    }
    if tokens[x] == "RPAREN"{
        return true;
    }
    if tokens[x] == "ASSIGN"{
        return true;
    }
    if tokens[x] == r""{
        return true;
    }
    let re = Regex::new(r"ID [a-z]+").unwrap();
    if re.is_match(&tokens[x]) {
    		return true;
    	}
    let re = Regex::new(r"NUM \d+").unwrap();
    if re.is_match(&tokens[x]) {
    		return true;
    	}
    return false;
}
//create parse tree function
/*This function creates a parse tree given the code from the original file*/
fn create_tree(name: &String, tokens: &Vec<&str>) -> std::io::Result<()>{

    //get name of file containing code, name parse tree file
    let re = Regex::new(r"\.").unwrap();
    let filename: Vec<&str> = re.split(name).collect();
    let ptfile = format!("{}.pt", filename[0]);
    let mut file = File::create(ptfile)?;

    //given tokens, find leaves
    let mut leafs = Vec::new();
    for x in 0..tokens.len()-2{
        let re = Regex::new(r"ID [a-z]+").unwrap();
        if re.is_match(&tokens[x]) {
        		leafs.push(tokens[x]);
        	}
        let re = Regex::new(r"NUM \d+").unwrap();
        if re.is_match(&tokens[x]) {
            	leafs.push(tokens[x]);
            }
    }

    //write to file in tree form
    file.write_all(b"START\n\tPOINT_DEF\n\t\t")?;
    file.write_all(leafs[0].as_bytes()).expect("Error writing to file");
    file.write_all(b" = POINT\n\t\t(\n\t\t")?;
    file.write_all(leafs[1].as_bytes()).expect("Error writing to file");
    file.write_all(b"\n\t\t,\n\t\t")?;
    file.write_all(leafs[2].as_bytes()).expect("Error writing to file");
    file.write_all(b"\n\t\t);")?;
    file.write_all(b"\n\tPOINT_DEF\n\t\t")?;
    file.write_all(leafs[3].as_bytes()).expect("Error writing to file");
    file.write_all(b" = POINT\n\t\t(\n\t\t")?;
    file.write_all(leafs[4].as_bytes()).expect("Error writing to file");
    file.write_all(b"\n\t\t,\n\t\t")?;
    file.write_all(leafs[5].as_bytes()).expect("Error writing to file");
    file.write_all(b"\n\t\t);")?;
    file.write_all(b"\n\tPOINT_DEF\n\t\t")?;
    file.write_all(leafs[6].as_bytes()).expect("Error writing to file");
    file.write_all(b" = POINT\n\t\t(\n\t\t")?;
    file.write_all(leafs[7].as_bytes()).expect("Error writing to file");
    file.write_all(b"\n\t\t,\n\t\t")?;
    file.write_all(leafs[8].as_bytes()).expect("Error writing to file");
    file.write_all(b"\n\t\t).")?;
    Ok(())
}
