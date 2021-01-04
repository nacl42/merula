use crate::Memo;
use crate::Node;

pub fn setup_memos() -> Vec<Memo> {
    let mut memos: Vec<Memo> = vec!();
        
    let memo = Memo::new("book", "The Lord of the Rings")
        .with(("author", "J.R.R. Tolkien"))
        .with(("character", "Bilbo Baggins"))
        .with(("character", "Samweis Gamdschie"))
        .with(("character", "Frodo Baggins"))
        .with(("character", "Aragorn"))
        .with(("character", "Legolas"))
        .with(("character", "Galadriel"));
    memos.push(memo);

    let memo = Memo::new("author", "J.R.R. Tolkien")
        .with(("birthday", "1892-01-03"));
    memos.push(memo);

    let memo = Memo::new("character", "Bilbo Baggins")
        .with(("species", "hobbit"))
        .with(("friend-of", "Samweis Gamdschie"))
        .with(("is-hobbit", true));
    memos.push(memo);

    let memo = Memo::new("character", "Samwise Gamgee")
        .with(("is-hobbit", true))
        .with(("species", "hobbit"));
    memos.push(memo);

    let memo = Memo::new("character", "Frodo Baggins")
        .with(("species", "hobbit"));
    memos.push(memo);
    
    let memo = Memo::new("character", "Aragorn")
        .with(("species", "men"));
    memos.push(memo);

    let memo = Memo::new("character", "Legolas")
        .with(("species", "sindar elf"))
        .with(("gender", "male"));
    memos.push(memo);

    let memo = Memo::new("character", "Galadriel")
        .with(("species", "elves"))
        .with(("gender", "female"));
    memos.push(memo);
        
    let memo = Memo::new("book", "The Hitchhiker's Guide to the Galaxy")
        .with(("author", "Douglas Adams"))
        .with(("character", "Arthur Dent"))
        .with(("character", "Ford Prefect"));        
    memos.push(memo);

    let memo = Memo::new("author", "Douglas Adams")
        .with(("birthday", "1952-03-11"));
    memos.push(memo);
    
    let memo = Memo::new("character", "Arthur Dent")
        .with(("gender", "male"))
        .with(("species", "human"))
        .with(("occupation", "BBC Radio employee"));
    memos.push(memo);

    let memo = Memo::new("character", "Ford Prefect")
        .with(("gender", "male"))
        .with(("species", "betelgeusian"))
        .with(("occupation", "Researcher for the Hitchhiker's Guide to the Galaxy"));
    memos.push(memo);

    memos
}
