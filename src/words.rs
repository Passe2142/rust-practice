#![allow(dead_code, unused)]

pub fn get_words() -> [&'static str; 100]{

        let mut words: [&str;100] = ["lace",  "            notebook",  "            grape",  "            grease",  "            ruddy",  "            graceful",  "            toy",  "            books",  "            paint",  "            tank",  "            squeeze",  "            trick",  "            cover",  "            bow",  "            impossible",  "            liquid",  "            delightful",  "            crazy",  "            land",  "            dislike",  "            rabbits",  "            fast",  "            screw",  "            pour",  "            absent",  "            practice",  "            dust",  "            yell",  "            succinct",  "            furtive",  "            squeak",  "            fill",  "            sand",  "            contain",  "            corn",  "            organic",  "            roomy",  "            poor",  "            scarecrow",  "            copy",  "            elated",  "            analyze",  "            cluttered",  "            melt",  "            obeisant",  "            rod",  "            flagrant",  "            comparison",  "            romantic",  "            wipe",  "            spell",  "            remind",  "            cure",  "            flame",  "            honey",  "            hesitant",  "            eight",  "            crime",  "            pretend",  "            earsplitting",  "            tenuous",  "            bite",  "            thundering",  "            plan",  "            replace",  "            place",  "            consist",  "            frighten",  "            confused",  "            swing",  "            profit",  "            synonymous",  "            cough",  "            dead",  "            abiding",  "            educate",  "            calendar",  "            record",  "            ambitious",  "            worry",  "            chess",  "            scared",  "            wax",  "            thank",  "            discover",  "            living",  "            illustrious",  "            achiever",  "            grumpy",  "            ignorant",  "            dynamic",  "            various",  "            sky",  "            nod",  "            lowly",  "            nation",  "            rot",  "            adventurous",  "            view",  "            icky"];
    for word in &mut words {
        *word = word.trim_start();
    }
    words
}

pub fn get_questions() -> [String;10]  {
    let questions :[String;10] = [
    "What are data types?".to_string(),
    "How does ownership work in Rust?".to_string(),
    "Explain the borrow checker in Rust.".to_string(),
    "What are the differences between structs and enums?".to_string(),
    "How are lifetimes used in Rust?".to_string(),
    "Describe Rust's error handling mechanisms.".to_string(),
    "What is pattern matching in Rust?".to_string(),
    "Explain Rust's concurrency model.".to_string(),
    "How does Rust manage memory?".to_string(),
    "What are traits in Rust?".to_string(),
    ];
    questions
}



pub fn get_answers() ->[String;10] {
    let answers : [String;10] = [
        "Data types are a fundamental concept that categorizes and specifies the type of data that a particular variable or value can hold. Data types define the kind of information that can be stored and manipulated within a program, and they help determine how the data is stored in memory.".to_string(),
        "Ownership in Rust is a system for managing memory and ensuring memory safety.".to_string(),
        "The borrow checker in Rust enforces ownership and borrowing rules to prevent data races and other memory-related issues.".to_string(),
        "Structs are used to create custom data types, while enums are used to define types with multiple possible values.".to_string(),
        "Lifetimes are used to ensure references in Rust are always valid and do not lead to data races.".to_string(),
        "Rust uses the Result and Option types for error handling, and the panic! macro for unrecoverable errors.".to_string(),
        "Pattern matching is a powerful feature in Rust used for deconstructing and matching data against patterns.".to_string(),
        "Rust's concurrency model allows for safe, concurrent code through ownership and borrowing rules.".to_string(),
        "Rust uses a combination of stack and heap memory management to ensure safety and performance.".to_string(),
        "Traits define methods and behavior that types can implement to provide shared functionality.".to_string(),
    ];
    answers
}

pub fn get_questions_and_answers() -> (Vec<String>, Vec<Vec<(String, bool)>>){

    let questions = vec!
    [
        "What are data types?".to_string(),
        "How does ownership work in Rust?".to_string(),
        "Explain the borrow checker in Rust.".to_string(),
        "What are the differences between structs and enums?".to_string(),
    ];
    let answers = vec!
    [
            vec![
            ("Data types are a fundamental concept that categorizes and specifies the type of data that a particular variable or value can hold. Data types define the kind of information that can be stored and manipulated within a program, and they help determine how the data is stored in memory.".to_string(), true),
            ("Rust uses a combination of stack and heap memory management to ensure safety and performance.".to_string(), false),
            ("The borrow checker in Rust enforces ownership and borrowing rules to prevent data races and other memory-related issues.".to_string(), false),
            ],
            vec![
            ("Traits define methods and behavior that types can implement to provide shared functionality.".to_string(), false),
            ("Ownership in Rust is a system for managing memory and ensuring memory safety.".to_string(), true),
            ("The borrow checker in Rust enforces ownership and borrowing rules to prevent data races and other memory-related issues.".to_string(), false),
            ],
            vec![
            ("Pattern matching is a powerful feature in Rust used for deconstructing and matching data against patterns.".to_string(), false),
            ("Rust uses the Result and Option types for error handling, and the panic! macro for unrecoverable errors.".to_string(), false),
            ("The borrow checker in Rust enforces ownership and borrowing rules to prevent data races and other memory-related issues.".to_string(), true),
            ],
            vec![
            ("Rust's concurrency model allows for safe, concurrent code through ownership and borrowing rules.".to_string(), false),
            ("Traits define methods and behavior that types can implement to provide shared functionality.".to_string(), false),
            ("Structs are used to create custom data types, while enums are used to define types with multiple possible values.".to_string(), true),
            ],
    ];
    (questions, answers)
}
