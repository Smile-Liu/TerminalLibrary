pub mod rt;

use std::{io::stdin, fmt};

fn main() {
    println!("Hello, world!");

    // 全局状态数据
    let mut global_data: Global = Global::new();
    
    println!("======================================");
    println!("== Welcome to The Bookstore System. ==");
    println!("======================================");
    println!("Please input command like -> mycommands. Tap 'exit' to exit the system.");
    loop {
        // 用户输入
        let mut line = String::new();

        stdin().read_line(&mut line).unwrap();

        if line.len() == 0 {
            println!("Please tap any words. Tap 'exit' to exit the system.");
            continue;
        }

        let arr: Vec<&str> = line.trim().split(" ").collect();
        let cmd = match arr.first() {
            Some(c) => match *c {
                "login" => {
                    if arr.len() != 3 {
                        println!("command usage: login <username> <password>.");
                        continue;
                    }
                    Command::Login { username: String::from(arr[1]), password: String::from(arr[2]) }
                },
                "logout" => {
                    Command::Logout
                },
                "user" => {
                    if arr.len() != 4 {
                        println!("command usage: user <username> <password> <role>.");
                        continue;
                    }
                    Command::User { username: String::from(arr[1]), password: String::from(arr[2]), role: String::from(arr[3]) }
                },
                "userdel" => {
                    if arr.len() != 2 {
                        println!("command usage: userdel <username>.");
                        continue;
                    }
                    Command::UserDel { username: String::from(arr[1]) }
                },
                "userlist" => {
                    let username = if arr.len() != 2 { String::from("") } else { String::from(arr[1]) };
                    Command::UserList { username }
                },
                "book" => {
                    if arr.len() != 3 {
                        println!("command usage: book <bookname> <author>.");
                        continue;
                    }
                    Command::Book { book: Book::new(String::from(arr[1]), String::from(arr[2]) ) }
                },
                "bookdel" => {
                    if arr.len() != 3 {
                        println!("command usage: bookdel <bookname> <author>.");
                        continue;
                    }
                    Command::BookDel { bookname: String::from(arr[1]), author: String::from(arr[1]) }
                },
                "booklist" => {
                    let bookname = if arr.len() != 2 { String::from("") } else { String::from(arr[1]) };
                    Command::BookList { bookname }
                },
                "borrow" => {
                    if arr.len() != 3 {
                        println!("command usage: borrow <bookname> <author>.");
                        continue;
                    }
                    Command::Borrow { bookname: String::from(arr[1]), author: String::from(arr[2]) }
                },
                "return" => {
                    if arr.len() != 3 {
                        println!("command usage: return <bookname> <author>.");
                        continue;
                    }
                    Command::Return { bookname: String::from(arr[1]), author: String::from(arr[2]) }
                }
                "mycommands" => {
                    if arr.len() != 1 {
                        println!("command usage: mycommands.");
                        continue;
                    }
                    Command::MyCommands
                },
                "exit" => break,
                _ => {
                    println!("Unsupported command. You can tap 'mycommands' to see all commands.");
                    continue;
                }
            },
            None => {
                println!("Please tap any words.");
                continue;
            }
        };
        let result = cmd.exec(&mut global_data);
        println!("> {}", result.unwrap_or("".to_string()));
    }
}

#[derive(Debug, Clone)]
enum Command {
    Login { username: String, password: String },
    Logout,
    User { username: String, password: String, role: String },
    UserDel { username: String },
    UserList { username: String },
    Book { book: Book },
    BookDel { bookname: String, author: String },
    BookList { bookname: String },
    Borrow { bookname: String, author: String },
    Return { bookname: String, author: String },
    MyCommands,
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Login { .. }, Self::Login { .. }) => true,
            (Self::Logout { .. }, Self::Logout { .. }) => true,
            (Self::User { .. }, Self::User { .. }) => true,
            (Self::UserDel { .. }, Self::UserDel { .. }) => true,
            (Self::UserList { .. }, Self::UserList { .. }) => true,
            (Self::Book { ..}, Self::Book { ..}) => true,
            (Self::BookDel { .. }, Self::BookDel { .. }) => true,
            (Self::BookList { .. }, Self::BookList { .. }) => true,
            (Self::Borrow { .. }, Self::Borrow { .. }) => true,
            (Self::Return { .. }, Self::Return { .. }) => true,
            (Self::MyCommands, Self::MyCommands) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Login { .. } => write!(f, "login"),
            Command::Logout { .. } => write!(f, "logout"),
            Command::User { .. } => write!(f, "user"),
            Command::UserDel { .. } => write!(f, "userdel"),
            Command::UserList { .. } => write!(f, "userlist"),
            Command::Book { .. } => write!(f, "book"),
            Command::BookDel { .. } => write!(f, "bookdel"),
            Command::BookList { .. } => write!(f, "booklist"),
            Command::Borrow { .. } => write!(f, "borrow"),
            Command::Return { .. } => write!(f, "return"),
            Command::MyCommands => write!(f, "mycommands"),
        }
    }
}

impl Command {
    
    fn exec(&self, global_data: &mut Global) -> Option<String> {
        if let Some(current_user) = global_data.get_current_user() {
            if !current_user.role.funcs.iter().any(|x| x == self) {
                return Some("You don't have permission to do this.".to_string());
            }
        }
        match self {
            Command::Login { username, password } => self.login(username, password, global_data),
            Command::Logout => self.logout(global_data),
            Command::User { username, password, role } => self.user(username, password, role, global_data),
            Command::UserDel { username } => self.user_del(username, global_data),
            Command::UserList { username } => self.user_list(username, global_data),
            Command::Book { book } => self.book(book, global_data),
            Command::BookDel { bookname, author } => self.book_del(bookname, author, global_data),
            Command::BookList { bookname } => self.book_list(bookname, global_data),
            Command::Borrow { bookname, author} => self.borrow(bookname, author, global_data),
            Command::Return { bookname, author } => self.return_book(bookname, author, global_data),
            Command::MyCommands => self.mycommands(global_data),
        }
    }

    fn login(&self, username: &String, password: &String, global_data: &mut Global) -> Option<String> {
        // 支持覆盖登录
        let mut me = User { username: username.to_string(), password: password.to_string(), role: Role::anonymous()};
        if username == "admin" && password == "123456" {
            me.role = Role::admin();
            let admin_user = Some(me);
            global_data.set_current_user(admin_user);
            return Some(String::from("Admin 'admin' login success."));
        } else {
            if let Some(login_user) = global_data.get_user_by_name(username) {
                if login_user.password == *password {
                    me.role = login_user.role.clone();
                    let user = Some(me);
                    global_data.set_current_user(user);
                    return Some(format!("{} '{}' login success.", login_user.role.rolename, username));
                }
                return Some(String::from("username or password error."));
            } else {
                return Some(String::from("User not exist."));
            }
        }
    }

    fn logout(&self, global_data: &mut Global) -> Option<String> {
        if let Some(user) = global_data.get_current_user() {
            global_data.set_current_user(Some(User::default_user()));
            return Some(format!("{} logout success.", user.username));
        }
        return Some(String::from("User not login."));
    }

    fn user_list(&self, username: &String, global_data: &mut Global) -> Option<String> {
        let users = global_data.list_user_by_name(username);

        let mut result = String::from("user:\n");
        for (i, ele) in users.iter().enumerate() {
            result += &format!("{} {} {}\n", i + 1, ele.username, ele.role.rolename);
        }
        Some(result)
    }

    fn user(&self, username: &String, password: &String, rolename: &String, global_data: &mut Global) -> Option<String> {
        if let Some(role) = Role::one_by_name(rolename.to_string()) {
            global_data.save_or_update_user(&User {username: username.to_string(), password: password.to_string(), role: role.clone()});
            return Some(String::from("User saved."));
        }
        Some("please input a valid role, like 'anonymous', 'admin', 'normal'.".to_string())
    }

    fn user_del(&self, username: &String, global_data: &mut Global) -> Option<String> {
        global_data.del_user(username)
    }

    fn book(&self, book: &Book, global_data: &mut Global) -> Option<String> {
        global_data.save_or_update_book(book);
        Some(String::from("Book saved."))
    }

    fn book_del(&self, bookname: &String, author: &String, global_data: &mut Global) -> Option<String> {
        global_data.del_book(bookname, author)
    }

    fn book_list(&self, bookname: &String, global_data: &mut Global) -> Option<String> {
        let books = global_data.list_book_by_name(bookname);
        
        let mut result = String::from("book:\n");
        for (i, ele) in books.iter().enumerate() {
            result += &format!("{} {} {} {}\n", i + 1, ele.bookname, ele.author, if ele.status == 0 { "can borrow" } else { "borrowed" });
        }
        Some(result)
    }

    fn borrow(&self, bookname: &String, author: &String, global_data: &mut Global) -> Option<String> {
        if let Some(book) = global_data.get_book_by_name_author(bookname, author) {
            if book.status == 0 {
                let mut book = book;
                global_data.save_or_update_book(&book.borrow_it());
                Some(format!("Book {} {} borrowed.", bookname, author))
            } else {
                Some(format!("Book {} {} was borrowed by other person.", bookname, author))
            }
       } else {
           Some(format!("Book {} {} not found.", bookname, author))
       }
    }

    fn return_book(&self, bookname: &str, author: &str, global_data: &mut Global) -> Option<String> {
        if let Some(book) = global_data.get_book_by_name_author(bookname, author) {
            if book.status == 1 {
                let mut book = book;
                global_data.save_or_update_book(&book.return_it());
                Some(format!("Book {} {} returned.", bookname, author))
            } else {
                Some(format!("Book {} {} not borrowed.", bookname, author))
            }
       } else {
           Some(format!("Book {} {} not found.", bookname, author))
       }
    }

    fn mycommands(&self, global_data: &mut Global) -> Option<String> {
        let mut result = String::from("command:\n");
        if let Some(user) = global_data.get_current_user() {
            for (i, ele) in user.role.funcs.iter().enumerate() {
                result += &format!("{} {}\n", i + 1, ele);
            }
        }
        Some(result)
    }
}

// 角色
#[derive(Debug, Clone)]
struct Role {
    rolename: String,
    funcs: Vec<Command>,
}

impl Role {
    
    fn new(rolename: String, funcs: Vec<Command>) -> Role {
        Role {rolename, funcs}
    }

    fn one_by_name(rolename: String) -> Option<Role> {
        if rolename == "anonymous" {
            return Some(Role::anonymous());
        } else if rolename == "admin" {
            return Some(Role::admin());
        } else if rolename == "normal" {
            return Some(Role::normal());
        }
        None
    }

    fn anonymous() -> Role {
        Role::new(String::from("anonymous"),
            vec![
                Command::Login { username: String::from(""), password: String::new() },
                Command::MyCommands])
    }

    fn admin() -> Role {
        Role::new(String::from("admin"),
            vec![
                    Command::User { username: String::from(""), password: String::new(), role: String::from("") },
                    Command::UserDel { username: String::from("") }, 
                    Command::UserList { username: String::from("") },
                    Command::Book { book: Book { bookname: String::from(""), author: String::from(""), status: 0 } },
                    Command::BookDel { bookname: String::from(""), author: String::from("") },
                    Command::BookList { bookname: String::from("") }, 
                    Command::Logout,
                    Command::MyCommands])// 管理员
    }

    fn normal() -> Role {
        Role::new(String::from("normal"), 
            vec![
                Command::BookList { bookname: String::from("") }, 
                Command::Borrow { bookname: String::from(""), author: String::from("") },
                Command::Return { bookname: String::from(""), author: String::from("") },
                Command::Logout,
                Command::MyCommands])// 普通用户
    }
}

// 用户
#[derive(Debug, Clone)]
struct User {
    username: String, // 用户名
    password: String, // 密码
    role: Role, // 角色
}

impl User {
    fn default_user() -> User {
        User {
            username: String::from(""),
            password: String::from(""),
            role: Role::anonymous(),
        }
    }
}

// 书
#[derive(Debug, Clone)]
struct Book {
    bookname: String,
    author: String,
    status: i8,// 状态 0-可借 1-已借出
}

impl Book {
    
    fn new(bookname: String, author: String) -> Book {
        Book {bookname, author, status: 0}
    }

    fn borrow_it(&mut self) -> &mut Self {
        self.status = 1;
        self
    }

    fn return_it(&mut self) -> &mut Self {
        self.status = 0;
        self
    }
}

// 全局状态数据
#[derive(Debug)]
struct Global {
    current_user: Box<Option<User>>,
    all_users: Vec<User>,
    all_books: Vec<Book>,
}

impl Global {
    fn new() -> Global {
        Global {
            current_user: Box::new(Some(User::default_user())),
            all_users: vec![],
            all_books: vec![],
        }
    }

    fn get_current_user(&self) -> Option<User> {
        *self.current_user.clone()
    }

    fn set_current_user(&mut self, user: Option<User>) {
        self.current_user = Box::new(user);
    }

    fn get_user_by_name(&mut self, username: &str) -> Option<User> {
        for ele in self.all_users.iter() {
            if username == ele.username {
                return Some(ele.clone());
            }
        }
        None
    }

    fn list_user_by_name(&mut self, username: &str) -> Vec<User> {
        if username == "" {
            return self.all_users.clone();
        }
        let mut result = vec![];
        for ele in self.all_users.iter() {
            if ele.username.contains(username) {
                result.push(ele.clone());
            }
        }
        result
    }

    fn save_or_update_user(&mut self, user: &User) {
        if let Some(user) = self.get_user_by_name(&user.username) {
            let index = self.all_users.iter().position(|x| x.username == user.username).unwrap();
            self.all_users[index] = user;
        } else {
            self.all_users.push(user.clone());
        }
    }

    fn del_user(&mut self, username: &str) -> Option<String> {
        if let Some(user) = self.get_user_by_name(username) {
            let index = self.all_users.iter().position(|x| x.username == user.username).unwrap();
            self.all_users.remove(index);
            return Some("User deleted.".to_string());
        }
        Some("User not exist.".to_string())
    }

    fn get_book_by_name_author(&self, bookname: &str, author: &str) -> Option<Book> {
        for book in &self.all_books {
            if book.bookname == bookname && book.author == author {
                return Some(book.clone());
            }
        }
        None
    }

    fn save_or_update_book(&mut self, book: &Book) -> Option<String> {
        if let Some(b) = self.get_book_by_name_author(&book.bookname, &book.author) {
            return Some(String::from("Book already exists"));
        } else {
            self.all_books.push(book.clone());
        }
        Some(String::from("Book saved successfully"))
    }

    fn list_book_by_name(&mut self, bookname: &str) -> Vec<Book> {
        if bookname == "" {
            return self.all_books.clone();
        }
        let mut result = vec![];
        for b in &self.all_books {
            if b.bookname == bookname {
                result.push(b.clone());
            }
        }
        result
    }

    fn del_book(&mut self, bookname: &str, author: &str) -> Option<String> {
        if let Some((i, b)) = self.all_books.iter().enumerate().find(|&(i, b)| b.bookname == bookname && b.author == author) {
            if b.status == 1 {
                return Some(format!("{} {} already borrowed", b.bookname, b.author));
            }
            self.all_books.remove(i);
            return Some(String::from("Book deleted successfully"));
        }
        Some(String::from("Book not found"))
    }
}