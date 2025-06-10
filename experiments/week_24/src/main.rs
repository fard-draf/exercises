fn broken_example<'a>() {
    let result1;
    {
        let temp1 = String::from("temporary");

        result1 = &temp1;
        println!("{result1}");
    };

    let result2 = {
        let temp2 = String::from("temporary cloned");

        temp2.clone()
    };
    println!("{result2}");

    let temp3 = String::from("easy ref");
    let result3: &str = { &temp3 };

    println!("{result3}");

    let temp4 = String::from("lifetime");
    let result4 = { goes_and_out(&temp4) };
    println!("{result4}");
}

fn goes_and_out<'a>(input: &'a String) -> &'a str {
    &input
    // it's an elision rule, so I'm not suppose to use explicited lifetimes
}

struct Person<'a> {
    name: &'a str,
    age: u32,
}

impl<'a, 'b> Person<'a> {
    pub fn find_oldest(people: &'b [Person<'a>]) -> Option<&'b Person<'a>> {
        people.iter().max_by_key(|p| p.age)
    }

    pub fn get_name_ref(person: &'a Person<'a>) -> &'a str {
        person.name
    }

    pub fn create_summary(people: &'a [Person<'b>], prefix: &'a str) -> String {
        format!("{} {} people.", prefix, people.len())
    }
}

fn main() {
    broken_example();

    let people = [
        Person {
            name: "Oliver",
            age: 32,
        },
        Person {
            name: "Katarina",
            age: 47,
        },
        Person {
            name: "Dan",
            age: 12,
        },
    ];

    let oldest_person = Person::find_oldest(&people);
    if let Some(older_person) = oldest_person {
        println!("The older person is {}.", older_person.name)
    }

    let get_name = Person::get_name_ref(&people[1]);
    println!("Hello, I'm {}!", get_name);

    let summary = Person::create_summary(&people, "Here, there are");
    println!("{}", summary);
}
