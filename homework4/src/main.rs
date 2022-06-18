// Bài tập
// Cho ngữ cảnh như sau : Một ngôi trường cần lập danh
//sách thông tin sinh viên bao gồm tên sinh viên và điểm của sinh viên đó.
// với mục đích thống kê kiểm tra giáo dục của ngôi trường này

/*-----------------------------*/
// Gợi ý:
// Định nghĩa bằng struct, mọi người nên sử dụng HashMap
// Tại sao lại sử dụng HashMap và ko phải Vec
//https://doc.rust-lang.org/std/collections/struct.HashMap.html
// struct School {
//     students: HashMap<String, u32>
// }

// trong trường hợp này thì String : tên của sinh viên đó
// u32 là điểm số

// Một số yêu cầu như sau:

/*-----------------------------*/
//0. Tạo 1 function new() khởi tạo rỗng ban đầu cho danh sách

/*-----------------------------*/
//1. Có thể thêm thông tin của sinh viên gồm có tên và điểm
// Ví dụ: thêm tên "Alice" có 7 điểm, thêm tên "Bob" có 2 điểm, ...
// Gợi ý : định nghĩa hàm "add" implement cho Struct

/*-----------------------------*/
//2. Liệt kê các điểm số hiện tại mà trường đã cập nhập
// ví dụ :danh sách hiện tại gồm có [(Alice, 10), (Bob,4)]
//trả về là [4,10] (điểm số nên tăng dần và ko có duplicate)
// ví dụ: [(Alice, 10), (Bob,4), (Steve,4)] -> [4,10]

/*-----------------------------*/
//3. Liệt kê danh sách các học sinh có cùng 1 điểm số
// ví dụ: hiện tại danh sách gồm có (Alice, 3), (Bob, 10), (Charlie,3)
// liệt kê danh sách học sinh có cùng 3 điểm : [Alice, Charlie]

// Yêu cầu trả về: danh sách sinh viên là alphabet theo tên

// Gợi ý:
// ví dụ : Ban đầu [(Alice, 10), (Bob,2), (Eve,4), (Long,2)] -> [(Bob, 2), (Long,2), (Eve,4), (Alice,10)]
//định nghĩa hàm "find_student" có tham số là điểm số -> trả về danh sách các sinh viên có cùng điểm số mong muốn

// Các bạn phải vuợt qua một số test case như sau :

/*-----------------------------*/
//Test case 1: Khởi tạo đầu tiên danh sách phải rỗng

/*-----------------------------*/

// Test case 2:
//Thêm sinh viên có tên "Lee" với điểm số là 2
// thì tất cả các điểm số hiện có của trường là 2
//nếu thêm sinh viên khác "Nancy" với điểm số là 3
//thì các điểm số hiện tại là [2,3]

/*-----------------------------*/

// Test case 3:
// Giả sử danh sách hiện tại : [(Bob, 4), (Alice,4), (Tom,5)]
// với điểm số 4 thì ta có sinh viên nào: -> [Alice, Bob] not [Bob ,Alice]
// vì cần tên theo alphabet

/*-----------------------------*/
// Nếu mọi người làm xong rùi thì có thể làm advance hơn bằng cách
// sử dụng Generic type cho điểm số không nhất thiết phải U32 nữa mà có thể "A+", "B+" chẳng hạn (string)
/*-----------------------------*/

// Sườn thông tin cho mọi người dễ làm

use std::collections::HashMap;
fn main() {
    println!("Hello world!")
}

pub struct School<T> {
    pub students: HashMap<String, T>,
}

impl<T: Clone + Ord + PartialEq> School<T> {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: T, student: &str) {
        self.students.entry(student.to_string()).or_insert(grade);
    }

    pub fn grades(&self) -> Vec<T> {
        let mut grades: Vec<T> = self.students.clone().into_values().collect::<Vec<T>>();
        grades.sort_unstable();
        grades.dedup();
        grades
    }

    pub fn grade(&self, grade: T) -> Vec<String> {
        let mut students_list = self
            .students
            .iter()
            .filter_map(|(key, val)| {
                if val == &grade {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
        students_list.sort_unstable();
        students_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_school_have_no_student() {
        let school: School<String> = School::new();
        assert!(school.students.is_empty() == true);
    }
    #[test]
    fn insert_student_to_school() {
        let mut school1 = School::new();
        let student_a_name = String::from("Lee");
        let student_a_grade = 2;
        school1.add(student_a_grade, &student_a_name);
        assert!(*school1.students.get(&student_a_name).unwrap() == student_a_grade);

        let mut school2 = School::new();
        let student_b_name = String::from("Anni");
        let student_b_grade = "A1".to_string();
        school2.add(&student_b_grade, &student_b_name);
        assert!(*school2.students.get(&student_b_name).unwrap() == &student_b_grade);
    }

    #[test]
    fn get_grades_of_school() {
        let mut school = School::new();
        let student_a_name = String::from("Lee");
        let student_b_name = String::from("Nancy");
        let student_c_name = String::from("Alice");
        let student_a_grade = 2;
        let student_b_grade = 3;
        let student_c_grade = 3;
        school.add(student_a_grade, &student_a_name);
        school.add(student_b_grade, &student_b_name);
        school.add(student_c_grade, &student_c_name);
        assert!(school.grades() == vec![2, 3]);
    }

    #[test]
    fn get_student_with_same_grade_i32() {
        let mut school = School::new();
        let student_a_name = String::from("Lee");
        let student_b_name = String::from("Nancy");
        let student_c_name = String::from("Alice");
        let student_a_grade = 2;
        let student_b_grade = 3;
        let student_c_grade = 3;
        school.add(student_a_grade, &student_a_name);
        school.add(student_b_grade, &student_b_name);
        school.add(student_c_grade, &student_c_name);
        assert!(school.grade(student_b_grade) == vec!["Alice", "Nancy"]);
    }

    #[test]
    fn get_student_with_same_grade_string() {
        let mut school = School::new();
        let student_a_name = String::from("Lee");
        let student_b_name = String::from("Nancy");
        let student_c_name = String::from("Alice");
        let student_a_grade = "A1";
        let student_b_grade = "A2";
        let student_c_grade = "A2";
        school.add(student_a_grade, &student_a_name);
        school.add(student_b_grade, &student_b_name);
        school.add(student_c_grade, &student_c_name);
        assert!(school.grade(student_b_grade) == vec!["Alice", "Nancy"]);
    }
}
