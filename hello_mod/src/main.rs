mod nation {
    pub mod government {
        pub fn govern() {
            println!("this is government");
        }
    }

    mod congress {
        pub fn legislate() {
            println!("this is congress");
        }
    }

    pub mod court {
        pub fn judicial() {
            super::congress::legislate();
        }
    }
}

fn main() {
    println!("Hello, world!");
    crate::nation::government::govern();
    crate::nation::court::judicial();
}
