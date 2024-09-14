use movetable::PieceType;

mod bitboard;
mod movetable;

fn main() {
    todo!()
}

fn get_move_demo() {
    let movetable = movetable::MoveTable::default();

    if let Some(v) = movetable.table.get(&(PieceType::Queen, 0x8000000000000000)) {
        let mut acc = 0_u64;
        for n in v {
            acc |= n;
        }
        let bitstr = format!("{:064b}", acc);
        let mut count = 0;
        for c in bitstr.replace("0", ".").replace("1", "X").chars() {
            print!("{c}");
            count += 1;
            if count % 8 == 0 {
                println!();
            }
        }
    } else {
        eprintln!("Error!");
    }
}
