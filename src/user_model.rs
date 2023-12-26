
struct  UserModel{
    user_name: String,
    email: String,
    id: u32,
    password: String,
    file: std::result::Result<File, std::io::Error>,
    data: DataFrame,
}

impl UserModel {
    fn new() {

        
    }
    
}


pub fn file_app() {
    let file_path: &str = "uspop.csv";
    let city_data: std::result::Result<File, std::io::Error> = File::open(file_path);

    let df: DataFrame = DataFrame::default();

    run_basic();
    let user_a : UserModel = UserModel 
    { 
        user_name: "Jules".to_string(), 
        email: "jules@asd.com".to_string(), 
        id: 12345,
        password: "kl{jsk:.?(0kf".to_string(),
        file: city_data,
        data: df,

    };


    println!("User data: ");

}
