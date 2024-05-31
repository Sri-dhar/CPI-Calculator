use crate::semdata;
use crate::SHOW_SPI;

//create write data and read data function 
pub fn write_data(scale: f32) {
    std::fs::write("configs/scale.txt", scale.to_string()).expect("Unable to write file");
}
pub fn read_data() -> f32 {
    let data = std::fs::read_to_string("configs/scale.txt").expect("Unable to read file");
    let scale = data.parse::<f32>().unwrap();
    scale
}


pub fn calculate_spi_n(sem_no: f32, grades: Vec<f32>) -> f32 {
    let semesters = semdata::get_semesters(sem_no).unwrap();
    let mut spi = 0.0;
    let total_credit = semesters.total_credit;
    for i in 0..semesters.course_name.len() {
        spi += grades[i] * semesters.course_credit[i];
    }
    spi / total_credit
}

pub fn calc_spi(sem_no: f32, grades: Vec<String>) -> f32 {
    let mut grades_recieved_by_student: Vec<f32> = Vec::new();
    for i in 0..grades.len() {
        let grade: f32 = grades[i].parse().expect("Invalid grade");
        // println!("Grade of course {} is {}", i+1, grade);
        grades_recieved_by_student.push(grade);
    }
    let spi = calculate_spi_n(sem_no, grades_recieved_by_student);
    // println!("SPI of Semester {} is {:.3}", sem_no, spi);
    //make SHOW_SPI TRUE
    *SHOW_SPI.lock().unwrap() = true;
    spi
}

pub fn calculate_cpi_option3(x: f32, cpi_of_xminus1: f32, spi_of_x: f32) -> f32 {
    let semester_x = semdata::get_semesters(x).unwrap();
    let semester_x_minus_1 = semdata::get_semesters(x-1.0).unwrap();

    let credit_of_sem_x :f32 = semester_x.total_credit;
    let cum_sum_of_credit_till_sem_x_minus_1 :f32 = semester_x_minus_1.total_credit_till_sem;

    let cpi_of_sem_x = (cpi_of_xminus1 * cum_sum_of_credit_till_sem_x_minus_1 + spi_of_x * credit_of_sem_x) / (cum_sum_of_credit_till_sem_x_minus_1 + credit_of_sem_x);
    
    cpi_of_sem_x
}

