use std::io;
use std::io::Write;

fn main() {

    struct Company {
        username:String,
        password:String,
        year:u32,
        shares:f32,
        liabilities:f32,
    }

    impl Company {
        fn percentageLeverage(&self) -> f32{
            return self.liabilities / self.shares * 100.0;
        }
    }

    let cadbury = Company {
        username:String::from("cadb"),
        password:String::from("cadb1234"),
        year:1965,
        shares:15_000_000.0,
        liabilities:5_500_000.0,
    };

    let champion = Company {
        username:String::from("cham"),
        password:String::from("cham1234"),
        year:1974,
        shares:25_000_000.0,
        liabilities:8_000_000.0,
    };

    let dangote = Company {
        username:String::from("dang"),
        password:String::from("dang1234"),
        year:1970,
        shares:18_000_000.0,
        liabilities:10_000_000.0,
    };

    let flour = Company {
        username:String::from("flou"),
        password:String::from("flou1234"),
        year:1960,
        shares:32_000_000.0,
        liabilities:4_000_000.0,
    };

    let nestle = Company {
        username:String::from("nest"),
        password:String::from("nest1234"),
        year:1961,
        shares:8_000_000.0,
        liabilities:1_500_000.0,
    };

    let unilever = Company {
        username:String::from("unil"),
        password:String::from("unil1234"),
        year:1923,
        shares:37_000_000.0,
        liabilities:11_000_000.0,
    };

    let honeywell = Company {
        username:String::from("hone"),
        password:String::from("cadb1234"),
        year:1906,
        shares:34_000_000.0,
        liabilities:9_000_000.0,
    };

    let nigerian = Company {
        username:String::from("nige"),
        password:String::from("nige1234"),
        year:1946,
        shares:30_000_000.0,
        liabilities:12_000_000.0,
    };

    let companies = vec![cadbury, champion, dangote, flour, nestle, unilever, honeywell, nigerian];

    

    println!("Please input the username");
    let mut username1 = String::new();
    std::io::stdin().read_line(&mut username1).expect("Failed to read username");
    let  username2 = username1.trim();
    
    //  STAGE ONE

    let mut state = "invalidated";
    if username2.len() >= 3 && username2.len() <= 8 {
        for i in 0..companies.len() {
            if username2 == companies[i].username {
                println!("That is a valid username");
    
                println!("Please input the password");
                let mut password1 = String::new();
                std::io::stdin().read_line(&mut password1).expect("Failed to read username");
                let  password2 = password1.trim();
    
                if password2 == companies[i].password {
                    println!("You have been successfully logged in");
                    state = "validated";
                } else {
                    println!("The inputted password is incorrect");
                }
            }
        }
    } else {
        println!("The inputed username is incorrect")
    }
    
    //  STAGE TWO

    if state == "validated" {

        let mut file = std::fs::File::create("data.txt").expect("create failed");
        let mut leverages = std::fs::File::create("leverages.txt").expect("create failed");

        for i in 0..companies.len() {
            file.write_all(companies[i].username.as_bytes()).expect("write failed");
            file.write_all("\n".as_bytes()).expect("write failed");
            file.write_all(companies[i].year.to_string().as_bytes()).expect("write failed");
            file.write_all("\n".as_bytes()).expect("write failed");
            file.write_all(companies[i].shares.to_string().as_bytes()).expect("write failed");
            file.write_all("\n".as_bytes()).expect("write failed");
            file.write_all(companies[i].liabilities.to_string().as_bytes()).expect("write failed");
            file.write_all("\n".as_bytes()).expect("write failed");
            file.write_all(companies[i].percentageLeverage().to_string().as_bytes()).expect("write failed");
            file.write_all("\n".as_bytes()).expect("write failed");
            file.write_all("\n".as_bytes()).expect("write failed");
            file.write_all("\n".as_bytes()).expect("write failed");

            if companies[i].shares > 20_000_000.00 {
                leverages.write_all(companies[i].percentageLeverage().to_string().as_bytes()).expect("write failed");
                leverages.write_all("\n".as_bytes()).expect("write failed");
                leverages.write_all("\n".as_bytes()).expect("write failed");
            }
            if companies[i].liabilities < 10_000_000.00 {
                let percent:f32 = 5.00 / 100.00 * companies[i].percentageLeverage();
                println!("{}'s 5% leverage is {}", companies[i].username, percent);
            }
        }
    } else {
        println!("The inputed username is incorrect")
    }
}
