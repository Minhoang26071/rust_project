// 02-Rust-Introduction/0.Comments.md
    // Hàm main -> Hàm thực hiện đầu tiên 
fn main() {
    // In ra terminal 
    println!("Hello, World!"); // In ra terminal ghi chú 1 dòng
        /*
        In Hello
        In World
        ghi chú nhiều dòng
    */
    println!("Hello");
    println!("World");
//02-Rust-Introduction/1.Print-Output.md
    print!("Xin chào Rust Bootcamp!"); //chỉ hiển thị kết quả ra terminal
    println!("Xin chào Rust Bootcamp!"); //hiển thị kết quả ra terminal, đồng thời thêm ký tự xuống dòng
//02-Rust-Introduction/2.Variables.md
    /*
    Biến là đại diện cho giá trị
Biến mặc định là bất biến (immutable)
Để tạo biến có thể thay đổi -> sử dụng từ khoá mut */
let x = 5;
let mut y = 10;
y += 1;
println!("giá trị của biến x là {}, giá trị của biến y là {}", x,y);
//02-Rust-Introduction/3.Data-Types.md
/*Có 2 kiểu dữ liệu:
Scalar: lưu trữ đơn giá trị như integer, float, char, boolean, string
Compound: lưu trữ đa giá trị như Array, Tuple */
let bootcamp = "Rust Bootcamp";
let year = 2024;
let free = true;
println!("Giá trị của bootcamp là {}", bootcamp);
println!("Giá trị của year là {}", year);
println!("Giá trị của free là {}", free);
//i32 dương và âm
//u32 dương
const PI: f32 = 3.14; // hằng số là không thể thay đổi
let arr: [i32; 5] = [1, 2, 3, 4, 5]; //mảng dữ liệu
let first = arr[0];
let dis = arr[4];
println!("tham số trong mảng dữ liệu bao gồm {} và {} ", first, dis);
// 02-Rust-Introduction/4.Type-Casting.md
/*Ép kiểu dữ liệu
Chuyển đổi kiểu dữ liệu này sang kiểu dữ liệu khác
Dùng từ khoá as
Sẽ có các trường hợp không thể ép kiểu dữ liệu được */
let x = 100;
let y = x as u8;
println!("{x}");
println!("{y}");
//02-Rust-Introduction/5.Operators.md
/*Toán tử
Hỗ trợ toán tử cơ bản như +, -, *, /,... cho các phép toán số học
Các toán tử so sánh như ==, !=, >, <, ...
Các toán tử logic như &&, ||, !
Ngoài ra còn có nhiều toán tử khác */
let a= 10i32;
let b= 20;
let add = a + b;
let sub = a - b;
println!("add: {}", add);
println!("sub {}", sub);
//toan tu gan
let mut x = 10;
x = x + 1;
println!("x la {}", x);
// toan tu so sanh
let a = 10;
let b = 20;
let c = a > b;
println!("{c}");
// toan tu logic: and, or, not
let x = true;
let y = false;
let z = x && y;
let w = x || y;
let g = !x;
println!("z: {z}");
println!("w: {w}");
println!("g: {g}");
//03-Rust-Control-Flow/0.If-Else.md
/*If..Else
Thực hiện các lựa chọn dựa trên điều kiện
Điều kiện trong if phải là boolean
else if hoặc else
Ví dụ: Ta đánh giá mức điểm A, B ,C dựa trên tổng điểm của 1 môn học nào đó
Nếu tổng điểm lớn hơn 90 -> Điểm A
Nếu tổng điểm lớn hơn 75 -> Điểm B
Nếu tổng điểm lớn hơn 65 -> Điểm C */
let number = 10;
if number > 0 {
    println!("{} is greater than 0", number);
}
else if number == 0 {
    println!("x la so 0")
}
else {
    println!("x nho hon 0")
} 
println!("Kết thúc chương trình")

}


