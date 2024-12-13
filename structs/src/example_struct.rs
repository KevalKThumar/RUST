fn calculate_area(l: u32, b: u32) -> u32 {
    l * b
}

fn calculate_area_with_tuples(dimensions: (u32, u32)) -> u32 {
    let h = dimensions.0;
    let b = dimensions.1;
    h * b
}
#[derive(Debug)]
struct Area {
    length: u32,
    height: u32,
}

fn calculate_area_with_struct(rect: &Area) -> u32 {
    let l = rect.length;
    let h = rect.height;

    l * h
}

pub fn main_example() {
    /* Normal */
    let area = calculate_area(4, 6);
    println!("area is {area} ");
    /* With Tuples */
    let dimensions = (4, 5);
    let area = calculate_area_with_tuples(dimensions);
    println!("area is {area} ");
    /* with Structure */
    let rect = Area {
        height: 45,
        length: 55,
    };
    let area = calculate_area_with_struct(dbg!(&rect));
    println!("The area of Rectangle {:#?} is {area}.",rect);

    /*
        :? is use to print debug value 
        :#? is use to print debug value in preety formate
     */

}
