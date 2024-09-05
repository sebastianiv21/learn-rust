#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn from_email(email: &str) -> Self {
        // let parts = email.split('@').collect::<Vec<&str>>();
        let parts: Vec<&str> = email.split('@').collect();
        // let username = parts.first().expect("username").to_string();
        let username = parts.first().unwrap().to_string();

        Self {
            username: username.clone(),
            email: email.to_string(),
            uri: format!("http://{}.com", username),
            active: true,
        }

        //     if let Some(new_username) = parts.first() {
        //         Self {
        //             username: new_username.to_string(),
        //             email: email.to_string(),
        //             uri: format!("http://{}.com", new_username),
        //             active: true,
        //         }
        //     } else {
        //         Self {
        //             username: "username".to_string(),
        //             email: email.to_string(),
        //             uri: "http://username.com".to_string(),
        //             active: true,
        //         }
        //     }
    }

    fn update_uri(&mut self, new_uri: &str) {
        self.uri = new_uri.to_string()
    }
}

fn main() {
    // let luis = User {
    //     username: "luisibarra".to_string(),
    //     email: "luis@example.com".to_string(),
    //     uri: "http://luisibarra.com".to_string(),
    //     active: true,
    // };
    //
    // let mut luis = User::new(
    //     "luisibarra".to_string(),
    //     "luis@example.com".to_string(),
    //     "http://luisibarra.com".to_string(),
    // );
    //
    // luis.deactivate();

    let mut luis = User::from_email("luis@example.com");

    println!("My user is {:#?}", luis); // {:#?} retorna el struct formateado, como pprint

    luis.update_uri("http://luisillo.com");

    println!("My new uri is {}", luis.uri);
}
