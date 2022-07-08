fn main() {
    let x = plus_one(5);
    println!("The value of x is {}", x);
}

fn plus_one(x: i32)  -> i32 {
    x + 1
}//;를 붙일 시 plus_one 함수는 return value가 없다

//7행 x + 1 에 ; 를 붙일 시 에러