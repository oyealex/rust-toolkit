struct Hold<T> {
    inner: T,
}

impl<T> Hold<T> {
    fn say(&self) {
        println!("say from gen");
    }
}

impl Hold<String> {
    fn say(&self) {
        println!("say from string");
    }
}


fn main() {
    let hold = Hold {
        inner: "123".to_string(),
    };
    hold.say();
}
