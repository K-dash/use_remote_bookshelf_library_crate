// cargo add --git https://github.com/K-dash/bookshelf_library.git
use my_library::library::{book::Book, bookshelf::Bookshelf};

fn main() {
    let mut shelf = Bookshelf::new();
    let book1 = Book::new("すごいぞChatGPT！AIを使って学ぼうRust！", "山田太郎");
    let book2 = Book::new("Pythonプログラミング入門", "山田花子");
    shelf.add_book(book1);
    shelf.add_book(book2);

    let title_query = "chatgpt";
    let found_books = shelf.search_books(title_query);
    println!("Found books: {:?}", found_books);
}
