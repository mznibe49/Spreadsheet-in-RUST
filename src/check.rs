use std::io::*;
use std::fs::File;
use regex::Regex;

/// Verifie l'extension des fichiers.
///
/// Plus précisément, verifie que chaque fichier est à son bon index
/// dans la ligne d'argument.
///
/// # Examples
///
/// ```
/// *.csv *.txt *.csv *.txt
/// ```
pub fn check_extension(vector: &mut Vec<&String>) -> std::io::Result<bool> {
    for i in 1..(vector.len()) {
        let file_name = vector[i];
        let tokens: Vec<&str> = file_name.split(".").collect();
        if (i % 2 == 0 && tokens[1] != "txt") || (i % 2 != 0 && tokens[1] != "csv") {
            return Ok(false);
        }
    }
    return Ok(true);
}

/// Verifie que le fichier data correspond à ce qui est attendu.
///
/// Par attendu, on entend un entier ou une formule.
pub fn check_data_file(file_name: &String) -> std::io::Result<bool> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    lazy_static! {
        static ref RE: Regex = Regex::new(r"^((=#\((\s*\d+\s*,){4}(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5])\)$)|(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5]))$").unwrap();
    }

    for line in reader.lines() {
        for element in line.unwrap().split(";") {
            let mut element_trim = String::from(element);
            element_trim.retain(|c| !c.is_whitespace());
            if !RE.is_match(&element_trim) {
                return Ok(false);
            }
        }
    }

    println!("OK !");

    return Ok(true);
}

/// Verifie que le fichier user correspond à ce qui est attendu.
///
/// Par attendu, on entend des lignes composées de:
/// entier entier (formule ou entier)
pub fn check_user_file(file_name: &String) -> std::io::Result<bool> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    lazy_static! {
        static ref RE_LINE: Regex = Regex::new(r"^(\d+) (\d+) (=#\((\s*\d+\s*,){4}\s*(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5])\s*\)|(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5]))$").unwrap();
    }

    for line in reader.lines() {
        match RE_LINE.captures(&line.unwrap()) {
            Some(_x) => {}
            None => return Ok(false)
        }
    }

    return Ok(true);
}

/// Verifie que le fichier view correspond à ce qui est attendu.
///
/// En cours de refonte.
pub fn check_view_file(filename: &String) -> std::io::Result<bool> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let int_reg = Regex::new(r"\d+").unwrap();

    for line in reader.lines() {
        for elt in line.unwrap().split(" ") {
            if !int_reg.is_match(&elt) && elt != "P" {
                return Ok(false);
            }
        }
    }

    return Ok(true);
}
