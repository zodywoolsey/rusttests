mod structtest;

fn main() {
    structtest::structtest();
    #[derive(PartialEq, Debug)]
    enum enumtest {
        empty,
        size_int32{size: i32},
        unnamed_triple_int32(i32, i32, i32),
        unnamed_triple_int32_again(i32, i32, i32),
    }
    println!("testing empty enum");
    let peep = enumtest::empty;
    println!("{}", peep == enumtest::empty);
    let aap = enumtest::unnamed_triple_int32(1, 13, 37);
    let aapp = enumtest::unnamed_triple_int32(1, 13, 38);
    let aappp = enumtest::unnamed_triple_int32(1, 13, 38);
    println!("{}",aap == aapp);
    println!("{}",aapp == aappp);
    match aap{
        enumtest::unnamed_triple_int32(x, y, z) => {
            println!("{}",x);
        },
        _ => {}
    }
}
