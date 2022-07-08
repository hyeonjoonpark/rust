fn main() {
    let var1: i32 = 1;
    var1 = 2;

    let mut var1: i32 = 1;
    var2 = 2;

    let var3: i32 = 1;
    let var3: i32 = 2;
    let var3: i32 = "str";

    const CON1:i32 = 1;
    CON1 = 2;
    const CON1:i32 = 2;
    const CON1:&str= "str";
}

//let을 사용하여 변수 생성
//let으로 생성된 변수는 값 수정 불가능
//let mut 로 생성된 변수는 수정 가능
//동일한 변수 이름을 let을 사용하여 여러번 생성 가능 - 값과 Type 모두 변경 될 수 있다.
//const를 통해 상수 만들 수 있고, 값과 Type 변경 불가능