/// Day 13: Mine Cart Madness
/// 
/// Parse and trace mine-cart tracks and figure out when they'll collide

// Note that +x is right and +y is down

#[derive(Clone)]
pub enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Clone)]
pub struct Cart {
    x: usize,
    y: usize,
    next_turn: Turn,
    direction: (isize, isize),
}

pub fn parse_map(map: &str) -> (Vec<Vec<char>>, Vec<Cart>) {
    let mut positions: Vec<Vec<char>> = vec![];
    let mut carts: Vec<Cart> = vec![];

    for (y, line) in map.lines().enumerate() {
        positions.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            if c == '>' {
                let cart = Cart { x, y, next_turn: Turn::Left, direction: (1, 0)};
                carts.push(cart);
                positions[y].push('-');
            } else if c == '<' {
                let cart = Cart { x, y, next_turn: Turn::Left, direction: (-1, 0) };
                carts.push(cart);
                positions[y].push('-');
            } else if c == '^' {
                let cart = Cart { x, y, next_turn: Turn::Left, direction: (0, -1) };
                carts.push(cart);
                positions[y].push('|');
            } else if c == 'v' {
                let cart = Cart { x, y, next_turn: Turn::Left, direction: (0, 1) };
                carts.push(cart);
                positions[y].push('|');
            } else {
                positions[y].push(c);
            }
        }
    }
    (positions, carts)
}

pub fn find_crash(map: &str) -> (usize, usize) {
    let (tracks, mut carts) = parse_map(map);
    loop {
        carts.sort_by_key(|cart| (cart.y, cart.x));

        for i in 0..carts.len() {
            {
                let mut cart = &mut carts[i];
                let (x, y) = cart.direction;
                cart.x = (cart.x as isize + x) as usize;
                cart.y = (cart.y as isize + y) as usize;
                let location = tracks[cart.y][cart.x];
                if location == '+' {
                    match cart.next_turn {
                        Turn::Left => {
                            cart.direction = turn_left(cart.direction);
                            cart.next_turn = Turn::Straight;
                        },
                        Turn::Straight => {
                            cart.next_turn = Turn::Right;
                        },
                        Turn::Right => {
                            cart.direction = turn_right(cart.direction);
                            cart.next_turn = Turn::Left;
                        }
                    }
                } else if location == '/' {
                    match cart.direction {
                        (0, -1) => cart.direction = (1, 0),
                        (-1, 0) => cart.direction = (0, 1),
                        (1, 0) => cart.direction = (0, -1),
                        (0, 1) => cart.direction = (-1, 0),
                        _ => panic!("Impossible direction"),
                    }
                } else if location == '\\' {
                    match cart.direction {
                        (1, 0) => cart.direction = (0, 1),
                        (0, 1) => cart.direction = (1, 0),
                        (-1, 0) => cart.direction = (0, -1),
                        (0, -1) => cart.direction = (-1, 0),
                        _ => panic!("Impossible direction"),
                    }
                }
            }    
            let cart = &carts[i];
            if carts.iter().filter(|c| c.x == cart.x && c.y == cart.y).count() > 1 {
                return (cart.x, cart.y);
            }
        }
    }
}

fn turn_left(direction: (isize, isize)) -> (isize, isize) {
    if direction == (1, 0) { (0, -1) }
    else if direction == (0, -1) { (-1, 0) }
    else if direction == (-1, 0) { (0, 1) }
    else { (1, 0) }
}

fn turn_right(direction: (isize, isize)) -> (isize, isize) {
    if direction == (1, 0) { (0, 1) }
    else if direction == (0, 1) { (-1, 0) }
    else if direction == (-1, 0) { (0, -1) }
    else { (1, 0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let map = "/->-\\        
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/   ";
    
        assert_eq!((7, 3), find_crash(map));
    }
}