// Partie 1 : afficher une grille de Sudoku
fn print_grid(grid: &[[u8; 9]; 9]) {
    println!();
    for l in 0..9 {
        if l > 0 && l % 3 == 0 {
            println!("---------+---------+---------");
        }
        for c in 0..9 {
            if c > 0 && c % 3 == 0 {
                print!("|")
            }
            match grid[l][c] {
                0 => print!("   "),
                value => print!(" {} ", value),
            }
        }
        println!();
    }
    println!();

    // let mut num_ligne: i32 = 0;
    // let mut num_colonne: i32 = 0;
    // println!();
    // for ligne in grid {
    //     num_ligne += 1;
    //     num_colonne = 0;
    //     let mut horizontale: String = "".to_string();
    //     for colonne in ligne {
    //         num_colonne += 1;
    //         if *colonne as i32 != 0 {
    //             horizontale += &String::from(" ".to_string() + &colonne.to_string());
    //         } else {
    //             horizontale += &String::from("  ");
    //         }

    //         if num_colonne % 3 == 0 && num_colonne != ligne.len() as i32 {
    //             horizontale += " |";
    //         }
    //     }
    //     println!("{}", horizontale);
    //     if num_ligne % 3 == 0 && num_ligne != ligne.len() as i32 {
    //         println!("-------+-------+------");
    //     }
    // }
}

// partie 2 : tester une proposition pour une case
fn is_possible(grid: &[[u8; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    for a in 0..9 {
        if grid[row][a] == num || grid[a][col] == num {
            return false;
        }
    }

    let debut_raw_carre = (row / 3) * 3;
    let debut_col_carre = (col / 3) * 3;

    for l in debut_raw_carre..=debut_raw_carre + 2 {
        for j in debut_col_carre..=debut_col_carre + 2 {
            if grid[l][j] == num {
                return false;
            }
        }
    }
    return true;
}

// partie 3 : explorer les grilles possibles et retourner les solutions
fn solve(grid: &[[u8; 9]; 9]) -> Vec<[[u8; 9]; 9]> {
    // initialisation du tableau des solutions trouvées
    let mut sols_founded: Vec<[[u8; 9]; 9]> = Vec::new();

    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c] == 0 {
                for i in 1..=9 {
                    if is_possible(&grid, r, c, i) {
                        let mut nouvelle_grille = grid.clone();
                        nouvelle_grille[r][c] = i;
                        //print_grid(&nouvelle_grille);

                        let solutions = solve(&nouvelle_grille);
                        sols_founded.extend(solutions)
                    }
                }
                return sols_founded;
            }
        }
    }
    sols_founded.push(*grid);
    return sols_founded;
}

fn main() {
    let grid: [[u8; 9]; 9] = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    print_grid(&grid);

    // for i in 1..9 {
    //     let value = is_possible(&grid, 0, 7, i);
    //     println!(" pour {i} => {value}");
    // }

    let results = solve(&grid);

    for r in &results {
        print_grid(&r);
    }

    println!("{} solution(s).", results.len());
}
