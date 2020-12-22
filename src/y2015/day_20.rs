
pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2015/20");
    let target_presents : usize = INPUT.parse().unwrap();

    let input = target_presents / 10;
    let mut house_number = input;
    let mut houses = vec![0;input];

    for i in 1..input {
        let mut elf = i;
        while elf < input {
            houses[elf] += i;
            if houses[elf] >= input && elf < house_number {
                house_number = elf;
            }
            elf += i;
        }
    }

    println!("[PART 1] House number {} will get {} presents", house_number, houses[house_number]*10);

    let input = target_presents / 11;
    let mut house_number = input;
    let mut houses = vec![0;input];

    for i in 1..input {
        let mut elf = i;
        let mut max_presents = 50;
        while elf < input {
            houses[elf] += i;
            max_presents -= 1;
            if houses[elf] >= input && elf < house_number {
                house_number = elf;
            }
            if max_presents == 0 {
                break;
            }
            elf += i;
        }
    }

    println!("[PART 2] House number {} will get {} presents", house_number, houses[house_number]*11);
}
