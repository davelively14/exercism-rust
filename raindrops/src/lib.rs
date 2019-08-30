#[derive(Debug, Clone)]
struct Result {
    val: String,
    n: u32
}

impl Result {
    fn new(n: u32) -> Result {
        Result {
            val: String::new(),
            n: n
        }
    }

    fn pling(&mut self) -> &mut Result {
        eval(self, "Pling", 3)
    }

    fn plang(&mut self) -> &mut Result {
        eval(self, "Plang", 5)
    }

    fn plong(&mut self) -> &mut Result {
        eval(self, "Plong", 7)
    }

    fn to_string(&mut self) -> String {
        match self.val.len() {
            0 => self.n.to_string(),
            _ => String::from(self.val.trim())
        }
    }
}

fn eval<'a>(result: &'a mut Result, word: &str, div: u32) -> &'a mut Result {
    match result.n % div {
        0 => {
            result.val = String::from(result.val.trim()) + word;
            result
        },
        _ => result
    }
}

pub fn raindrops(n: u32) -> String {
    let mut result = Result::new(n);

    result
        .pling()
        .plang()
        .plong()
        .to_string()
}