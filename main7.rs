fn main() {
    let x: i32 = 5;

    let x: i32 = x + 1;

    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {}", x); //값 12
    }//Local 영역에서 사용되는 변수
    

    println!("The value of x is: {}", x); //값 6
}