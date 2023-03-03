fn main() {
    let blocktop = "------------";  
    let blockbody = "|            ";    // block widths as square



    // learn structs
    let wpawn = "  0\n |||\n |||\n |||\n|||||";
    let bpawn = "  0\n | |\n | |\n | |\n|   |";
    let wrook = "R";
    let brook = "R";
    let wknight = "N";
    let bknight = "N";
    let wbishop = "B";
    let bbishop = "B";
    let wqueen = "Q";
    let bqueen = "Q";
    let wking = "K";
    let bking = "K";


    render_board(blocktop, blockbody);


}


fn render_board(blocktop: &str, blockbody: &str) {
    for letter in 1..=8 {  // print block number down

        for i in 0..=8 {
            print!("{blocktop}");
        }
        print!("\n");
        for i in 0..= 4 {  // print block height
            for number in 0..=8 {  // print block number across

                // if piece exists, print piece
                // render piece by splitting the piece's signature at \n into a slice which inserts in the center of blockbody
                // else, print blockbody


                // if number == 0, add rank number
                if number == 0 {
                    if i == 2 {
                        print!("{} {blockbody}", letter, blockbody=blockbody);
                    } else {
                        print!("  {blockbody}");
                    }
                } else {

                print!("{blockbody}"); 
                }
            }
            print!("\n");
        }

    }
    for i in 0..=8 {  // print bottom row
        print!("{blocktop}");
    }
    print!("\n  ");
    for file in ["a","b","c","d","e","f","g","h"] {  // print letters across
        print!("      {file}      ");
    }
    print!("\n");
}






// white pawn
//   ()
//  /--\
//  \--/
//  /--\
// /----\

// black pawn
//   ()
//  /  \
//  \  /
//  /  \
// /    \

// white rook

// || || ||
// |------|
//  |----|
//  |----|
// |------|

// black rook
// || || ||
// |      |
//  |    |
//  |    |
// |      |

// white knight
//    ______
//   /--- o 
//  /L -----
//  \~~_____
//      \---

// black knight
//    ______
//   /    o 
//  /L      
//  \~~_____
//      \___

// white bishop
//      O
//     /-\
//    /---\
//   /- + -\
//  ~\-----/~

// black bishop
//      O
//     / \
//    /   \
//   /  +  \
//  ~\     /~

// white queen
// O O O O O
// \-|-|-|-/
//  \-----/
//   \---/
//  =======

// black queen
// O O O O O
// \ | | | /
//  \     /
//   \   /
//  =======


// white king
//   __+__
//  /--|--\
//  \--|--/
//  /--|--\
// =========


// black king
//     __+__
//    /  |  \
//    \  |  /
//    /  |  \
//   =========
