Trong Rust, có hai loại kiểu dữ liệu chính:

**1. Kiểu dữ liệu nguyên thủy:**

* **Số nguyên:**
    * **Có dấu:** i8, i16, i32, i64, i128
    * **Không dấu:** u8, u16, u32, u64, u128
* **Số thực:** f32, f64
* **Kiểu Boolean:** bool
* **Ký tự:** char
* **Tuple:** ()

**2. Kiểu dữ liệu phức tạp:**

* **Mảng:** [T; n]
* **Cấu trúc (Struct):** struct MyStruct { field1: Type1, field2: Type2 }
* **Liên hợp (Union):** union MyUnion { field1: Type1, field2: Type2 }
* **Enum:** enum MyEnum { Variant1, Variant2 }
* **Chuỗi:** String
* **Vector:** Vec<T>
* **Slice:** &[T]
* **HashMap:** HashMap<K, V>

Ngoài ra, Rust còn có một số kiểu dữ liệu đặc biệt khác như:

* **Đóng gói (Wrapper):** Box<T>, Rc<T>
* **Con trỏ (Reference):** &T, &mut T
* **Mệnh đề (Closure):** || { ... }

Số lượng kiểu dữ liệu trong Rust có thể thay đổi tùy thuộc vào phiên bản của trình biên dịch. Tuy nhiên, đây là những kiểu dữ liệu cơ bản mà bạn cần nắm vững để bắt đầu lập trình với Rust.

**Lưu ý:**

* Kiểu dữ liệu nguyên thủy được lưu trữ trực tiếp trên bộ nhớ.
* Kiểu dữ liệu phức tạp được tạo ra từ các kiểu dữ liệu khác.
* Rust là ngôn ngữ lập trình có kiểu dữ liệu tĩnh, nghĩa là bạn phải khai báo kiểu dữ liệu cho mỗi biến.
## Phân tích kỹ nội dung về Scalar và Compound

**Scalar:**

* **Định nghĩa:** Scalar là kiểu dữ liệu lưu trữ một giá trị đơn lẻ.
* **Ví dụ:**
    * Số nguyên (integer): 1, 2, 3, ...
    * Số thực (float): 1.2, 3.14, ...
    * Ký tự (char): 'a', 'b', 'c', ...
    * Giá trị Boolean: true, false
    * Chuỗi (string): "Hello", "World", ...
* **Đặc điểm:**
    * Scalar chỉ có thể lưu trữ một giá trị duy nhất.
    * Scalar có kích thước cố định.
    * Scalar có thể được so sánh trực tiếp với nhau.

**Compound:**

* **Định nghĩa:** Compound là kiểu dữ liệu lưu trữ nhiều giá trị.
* **Ví dụ:**
    * Mảng (Array): [1, 2, 3], ["a", "b", "c"]
    * Tuple: (1, 2, 3), ("a", "b", "c")
* **Đặc điểm:**
    * Compound có thể lưu trữ nhiều giá trị thuộc cùng một kiểu dữ liệu hoặc khác nhau.
    * Compound có kích thước có thể thay đổi.
    * Compound không thể được so sánh trực tiếp với nhau.

**Bảng so sánh:**

| Đặc điểm | Scalar | Compound |
|---|---|---|
| Định nghĩa | Lưu trữ một giá trị đơn lẻ | Lưu trữ nhiều giá trị |
| Ví dụ | Số nguyên, số thực, ký tự, Boolean, chuỗi | Mảng, Tuple |
| Kích thước | Cố định | Có thể thay đổi |
| So sánh | Có thể so sánh trực tiếp | Không thể so sánh trực tiếp |

**Ví dụ áp dụng:**

* **Scalar:** Sử dụng để lưu trữ các giá trị đơn lẻ như số lượng sản phẩm trong giỏ hàng, điểm thi của một học sinh, tên của một người.
* **Compound:** Sử dụng để lưu trữ các tập hợp dữ liệu như danh sách sản phẩm trong cửa hàng, thông tin cá nhân của một người, kết quả của một cuộc khảo sát.

**Kết luận:**

Scalar và Compound là hai kiểu dữ liệu cơ bản trong lập trình. Việc lựa chọn kiểu dữ liệu nào để sử dụng phụ thuộc vào nhu cầu cụ thể của từng bài toán.
