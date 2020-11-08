struct LazySum {
    a: u32,
    b: u32,
    sum: Option<u32>,
}

impl LazySum {
    fn new(a: u32, b: u32) -> Self {
        Self { a, b, sum: None }
    }

    fn calculate(&mut self) -> &u32 {
        // self.sum.get_or_insert(self.a + self.b)
        match self.sum {
            Some(ref sum) => {
                println!("sum exists, return:");
                sum
            }
            None => {
                println!("sum doesn't exist, calculate:");
                self.sum.replace(self.a + self.b);
                self.sum.as_ref().unwrap()
            }
        }
    }
}

struct LazyJoin {
    a: Vec<String>,
    b: Vec<String>,
    result: Option<Vec<String>>,
}

impl LazyJoin {
    fn new(a: Vec<String>, b: Vec<String>) -> Self {
        Self { a, b, result: None }
    }

    fn join(&mut self) -> &Vec<String> {
        // let a = &self.a;
        // let b = &self.b;
        // self.result.get_or_insert_with(|| {
        //     println!("result not found, join:");
        //     let mut a = a.clone();
        //     a.extend_from_slice(b);
        //     a
        // })
        match self.result {
            Some(ref result) => {
                println!("find result, return:");
                result
            }
            None => {
                println!("result not found, create:");
                let mut a = self.a.clone();
                a.extend_from_slice(&self.b);
                self.result.replace(a);
                self.result.as_ref().unwrap()
            }
        }
    }
}

fn main() {
    let mut s = LazySum::new(1, 2);
    println!("1 + 2 = {}", s.calculate());
    println!("1 + 2 = {}", s.calculate());
    let mut j = LazyJoin::new(vec![String::from("aaa")], vec![String::from("bbb")]);
    println!("['aaa'] + ['bbb'] = {:?}", j.join());
    println!("['aaa'] + ['bbb'] = {:?}", j.join());
}
