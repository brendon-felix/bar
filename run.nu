use std repeat



def print_bar [length p] {
    let inc_hex = $p * 255 | into int | format number | get upperhex | str substring 2.. | fill -w 2 -c '0' -a r
    let dec_hex = (1 - $p) * 255 | into int | format number | get upperhex | str substring 2.. | fill -w 2 -c '0' -a r
    let fg_color = '#' + $inc_hex + $dec_hex + '00';
    let ansi_type = {
        fg: $fg_color,
        bg: '#202020',
    }
    print -n $"(ansi -e $ansi_type)(./target/release/bar -l $length $p)(ansi reset)\r"; sleep 10ms

}

def main [length: int = 10] {
    # let length = 10

    let step = 1.0 / ($length * 8)

    loop {
        for p in (0..$step..1.0) {
            print_bar $length $p
        }
        for p in (1.0..(1.0 - $step)..0) {
            print_bar $length $p
        }
    }
}
