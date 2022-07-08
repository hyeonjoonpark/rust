fn main() {
    let mut/*mutable*/x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
/*
실행결과
The value of x is: 5
The value of x is: 6
*/
//변수를 선언할 때 수정할 지 말지 정의하고 사용함으로써 SW의 버그발생 확률을 낮추고 소스코드 가독성 개선