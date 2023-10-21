use std::f32::consts::E;
use std::fmt::Display;
use std::io::{self, stdout};
use std::collections::HashMap;
use std::{string, result};

/*
 * 课程
 */
#[derive(Debug, Clone)]
struct Course {
    name: String,
    compulsory: bool,  //true 必修课，false 选修课
    teacher_name: String,  //授课老师
}

/*
 * 社团
 */
#[derive(Debug, Clone)]
struct Club {
    name: String,   //社团名称
    content: String,  //社团职责内容
}

/*
 * 班级
 */
#[derive(Debug, Clone)]
struct GradeClass {
    name:String, //班级名称
    grade: String,  //年级
    faculty: String,   //院系
    speciality: String,   //专业
}

/*
 * 性别
 */
#[derive(Debug, PartialEq, Clone)]
enum Gender {
    Male,   // 男
    Female, // 女
}

use std::str::FromStr;

impl FromStr for Gender {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unit = match s {
            "man" => Gender::Male,
            "female" => Gender::Female,
            _ => return Err(s.to_string()),
        };
        Ok(unit)
    }
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        let ret = match self {
            Gender::Male => "man",
            Gender::Female => "female",
        };
        ret.to_string()
    }
}

/*
 * 学生
 */
#[derive(Debug, Clone)]
struct Student {
    name: String,  //姓名
    age: u8,   //年龄
    gender: Gender,   //性别
    gradeclass: GradeClass, //班级
    clubs: Vec<Club>, //参加的社团
    courses: Vec<Course>, //学习的课程
}

impl Student {
    //更改学生名
    fn change_name(&mut self, new_name:String) { 
       self.name = new_name;
    }

    //更改学生性别
    fn change_gender(&mut self, new_gender:String) { 
        let result = Gender::from_str(&new_gender);
        let gender_enum = result.unwrap();
        self.gender = gender_enum;
     }

     //删除某一个社团
     fn drop_club_by_number(&mut self, remove_club_number:usize) { 
        let club = self.clubs.get(remove_club_number-1).unwrap();
        self.clubs.remove(remove_club_number);
     }
}
/**
 * 第三课作业：
 * 请基于Rust的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作
 */
fn main() {
    println!("►►►► 第一步 创建课程 ◄◄◄◄");
    let mut course_vec = Vec::new();
    create_course(&mut course_vec);
    println!("►►►► 课程列表如下: ◄◄◄◄");
    for index in 0..course_vec.len() {
        println!("编号{}, 课程信息：{:?}", index+1, course_vec.get(index).unwrap());
    }

    println!("►►►► 第二步 创建社团: ◄◄◄◄");
    let mut club_vec = Vec::new();
    create_club(&mut club_vec);
    println!("►►►► 社团列表如下: ◄◄◄◄");
    for index in 0..club_vec.len() {
        println!("编号{}, 社团信息：{:?}", index+1, club_vec.get(index).unwrap());
    }

    println!("►►►► 第三步 创建班级: ◄◄◄◄");
    let mut gradeclass_vec = Vec::new();
    create_gradeclass(&mut gradeclass_vec);
    println!("►►►► 班级列表如下: ◄◄◄◄");
    for index in 0..gradeclass_vec.len() {
        println!("编号{}, 班级信息：{:?}", index+1, gradeclass_vec.get(index).unwrap());
    }

    println!("►►►► 第四步 添加学生（以1个学生为例）: ◄◄◄◄");
    let mut student = create_student(&course_vec, &club_vec, &gradeclass_vec);
    println!("►►►► 学生信息如下: ◄◄◄◄");
    println!("{:?}", student);

    println!("►►►► 第五步 修改学生信息 ◄◄◄◄");
    println!("►►►► 修改学生名称->韩梅梅 ◄◄◄◄");
    student.change_name("韩梅梅".to_string());

    println!("►►►► 修改学生性别->女 ◄◄◄◄");
    student.change_gender("female".to_string());

    println!("►►►► 删除课程1 ◄◄◄◄");
    student.drop_club_by_number(1);

    println!("►►►► 修改后的学生信息如下: ◄◄◄◄");
    println!("{:?}", student);

}

/**
 * 创建学生 
 */
fn create_student(course_vec: &Vec<Course>, club_vec: &Vec<Club>, gradeclass_vec: &Vec<GradeClass>) -> Student{
    println!("请输入学生名:");
    let mut input = String::new();        
    io::stdin().read_line(&mut input).expect("error");
    let name: String = input.trim().to_string();
    //
    println!("请输入性别(man or female):");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("error");
    let gender: String = input1.trim().to_string();
    let result = Gender::from_str(&gender);
    let gender_enum = result.unwrap();

    println!("请输入年龄:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("error");
    let age_str: String = input2.trim().to_string();
    let age = age_str.parse::<u8>().unwrap();

    println!("请选择下列班级（请输入编号）:");
    for index in 0..gradeclass_vec.len() {
        println!("    ->>>> 编号{}, 班级名称：{:?}", index+1, gradeclass_vec.get(index).unwrap());
    }
    println!("请输入班级编号:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("error");
    let index_str: String = input3.trim().to_string();
    let gradeclass_num = index_str.parse::<usize>().unwrap() - 1;
    let gradeclass = gradeclass_vec[gradeclass_num].clone();

    println!("请选择下列社团:");
    for index in 0..club_vec.len() {
        println!("    ->>>> 编号{}, 社团名称：{:?}", index+1, club_vec.get(index).unwrap());
    }
    println!("请输入社团编号（如若参与多个社团，请逗号分隔）:");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect("error");
    let index_str_list: String = input4.trim().to_string();
    //
    let mut clubs = Vec::new();
    for split_string in index_str_list.split(",") {
        let club_num = split_string.parse::<usize>().unwrap() - 1;
        let club = club_vec[club_num].clone();
        clubs.push(club);
    }

    println!("请选择下列课程:");
    for index in 0..course_vec.len() {
        println!("    ->>>> 编号{}, 课程名称：{:?}", index+1, course_vec.get(index).unwrap());
    }
    println!("请输入课程编号（如若参与多个课程，请逗号分隔）:");
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5).expect("error");
    let index_str_list: String = input5.trim().to_string();
    //
    let mut courses = Vec::new();
    for split_string in index_str_list.split(",") {
        let course_num = split_string.parse::<usize>().unwrap() - 1;
        let course = course_vec[course_num].clone();
        courses.push(course);
    }
    //
    let student = Student {
                                        name,
                                        age,
                                        gender:gender_enum,
                                        gradeclass,
                                        clubs,
                                        courses,
                                    };
    student
}

/**
 * 创建班级 如：2023届计算机科学班、2020届艺术美声班 等
 */
fn create_gradeclass(gradeclass_vec: &mut Vec<GradeClass>){
    loop {
        println!("请输入第几届:");
        let mut input = String::new();        
        io::stdin().read_line(&mut input).expect("error");
        let grade: String = input.trim().to_string();
        //
        println!("请输入院系:");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("error");
        let faculty: String = input1.trim().to_string();

        println!("请输入专业:");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("error");
        let speciality: String = input2.trim().to_string();
        //
        let name = format!("{}届{}班", grade, speciality);
        //
        let gradeClass = GradeClass {
                                            name,
                                            grade,
                                            faculty,
                                            speciality,
                                        };
        gradeclass_vec.push(gradeClass);
        //
        println!("是否继续创建班级（yes/no）:");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("error");
        let continue_create = input3.trim();
        if continue_create != "yes" {
            println!("班级创建完毕！");
            break;
        } 
        println!("继续创建班级。。。");
        
    };
}


/**
 * 创建社团 如：舞蹈社团、吉他社团、摄影社团 等
 */
fn create_club(club_vec: &mut Vec<Club>){
    loop {
        println!("请输入社团名称:");
        let mut input = String::new();        
        io::stdin().read_line(&mut input).expect("error");
        let name: String = input.trim().to_string();
        //
        println!("请输入社团职能:");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("error");
        //
        let club = Club {
                                name,
                                content: input2.trim().to_string(),
                            };
        club_vec.push(club);
        //
        println!("是否继续创建社团（yes/no）:");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("error");
        let continue_create = input3.trim();
        if continue_create != "yes" {
            println!("社团创建完毕！");
            break;
        } 
        println!("继续创建社团。。。");
        
    };
}

/**
 * 创建课程 如：英语，计算机，高等数学 等
 */
fn create_course(course_vec: &mut Vec<Course>){
    loop {
        println!("请输入课程名:");
        let mut input = String::new(); //课程名        
        io::stdin().read_line(&mut input).expect("error");
        let name: String = input.trim().to_string();
        //
        println!("是否是必修课（yes/no）:");
        let mut input1 = String::new(); //是否必修课
        io::stdin().read_line(&mut input1).expect("error");
        let mut compulsory = false;
        if "yes" == input1.trim() {
            compulsory = true;
        }
        //
        println!("请输入授课教师:");
        let mut input2 = String::new(); //是否必修课
        io::stdin().read_line(&mut input2).expect("error");
        //
        let course = Course {
                                        name,
                                        compulsory,
                                        teacher_name: input2.trim().to_string(),
                                    };
        course_vec.push(course);
        //
        println!("是否继续创建课程（yes/no）:");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("error");
        let continue_create = input3.trim();
        if "yes" == continue_create {
            println!("继续创建课程。。。");
        } else {
            println!("课程创建完毕！");
            break
        }
    };
}