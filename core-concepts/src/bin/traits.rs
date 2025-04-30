trait Greet{
    fn say_hello(&self);
}

struct Person{
    name:String,
}

impl Greet for Person {
    fn say_hello(&self){
        println!("Hi, I'm {}",self.name)
    }
}

fn main(){
    let p= Person{name:"Muhammed".into()};
    p.say_hello()
}


//Traits define shared behavior, like TypeScript interface or implements.

// interface Greet {
//     sayHello(): void;
//   }
  
//   class Person implements Greet {
//     name: string;
//     constructor(name: string) {
//       this.name = name;
//     }
  
//     sayHello() {
//       console.log(`Hi, I'm ${this.name}`);
//     }
//   }