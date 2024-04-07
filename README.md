# TerminalLibrary
This is a terminal system for library management which developed by Rust.

## Usage
1. Clone this repository.

2. Run `cargo run` in the root directory.

3. Follow the instructions to use the system.
    1. A default user: `admin` with password: `123456` can be used to login.
    ```rust
    login admin 123456
    ```

    2. 3 roles are available: `admin`, `normal`, `anonymous`.
    no login is anonymous user.
    admin user can do jobs like user management, book management, you will see user's available commands by `mycommands` command.
    normal user can do jobs like borrow books and return books, you will see user's available commands by `mycommands` command.

4. Here is a simple example of how to use.
```rust
login admin 123456
> Admin 'admin' login success.
user me 123 normal
> User saved.
userlist
> user:
1 me normal

user him 123 normal
> User saved.
userlist
> user:
1 me normal
2 him normal

book 标准日本语 我
> Book saved.
book Thine-in-Java Kimi-John
> Book saved.
booklist
> book:
1 标准日本语 我 can borrow
2 Thine-in-Java Kimi-John can borrow

logout
command usage: logout <username>.
logout admin
> admin logout success.
login him 123
> normal 'him' login success.
mycommands
> command:
1 borrow
2 return
3 logout
4 mycommands

borrow Think-in-Java Kimi-John
> Book Think-in-Java Kimi-John not found.
borrow Thine-in-Java Kimi-John
> Book Thine-in-Java Kimi-John borrowed.
```