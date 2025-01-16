struct Person {
    firstname: String,
    age: i32,
    active: bool,
}



fn main() {
    {
        let pers1 = Person {
            firstname: String::from("sigma"),
            active: true,
            age: 1
        };
        let newUser = build_user(String::from("josef"), 123);
        println!("{}", newUser.age);


        let sigma = "awdasd";
        let sigmaaa = String::from("adwd");
    }



    {
        let user1 = Person {
            firstname: String::from("OHIO"),
            age: 69,
            active: false
        };
        println!("name : {} age : {} active {} ", user1.firstname, user1.age, user1.active);


        let user2 = Person {
            firstname: String::from("Rizzler"),
            ..user1
        };
        println!("name : {} age : {} active {} ", user2.firstname, user2.age, user2.active);
    }

    {
        let rect_of_trust = Rectangle {
            width: 31,
            height: 31
        };
        println!("THE area {}", rect_of_trust.area());
        println!("OF: w:{} h:{}",rect_of_trust.width, rect_of_trust.height);
        println!("THE REC : {:#?}", &rect_of_trust);
    }
    {
        let rect0 = Rectangle {
            width: 123,
            height: 123
        };

        let rect1 = Rectangle {
            width: 100,
            height: 100
        };

        let rect2 = Rectangle {
            width: 150,
            height: 150,
        };


        println!("RECT CAN HOLD RECT1? : {}",rect0.can_hold(&rect1));
        println!("RECT CAN HOLD RECT2? : {}",rect0.can_hold(&rect2));

    }

}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.area() > other.area() {
            true
        } else {
            false
        }
    }
}



/*fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}*/


fn build_user(firstname: String, age: i32) -> Person {
    Person {
        firstname,
        age,
        active: true,
    }
}
