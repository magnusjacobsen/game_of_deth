use std::collections::HashMap;
use rand::prelude::*;

pub type Level = HashMap<(i64, i64), usize>;

pub fn get_level_1() -> Level {
    let cells = vec![(4,-4),(4,-1),(7,2),(8,1),(10,2),(3,7),(3,-2),(-1,3),(5,-2),(3,-3),(-1,1),(1,2),(0,1),(9,1),(0,3),(5,7),(5,6),(3,6),(4,5),(-2,2),(9,3),(5,-3),(8,3),(4,8)];
    create_level_from_vec(cells)
}

pub fn get_level_2() -> Level {
    let cells = vec![(1,2),(-1,-1),(-2,-2),(-2,-1),(1,-4),(2,0),(5,-1),(2,3),(3,-4),(6,-1),(2,-3),(3,1),(-2,0),(6,0),(1,1),(3,2),(6,-2),(2,-4)];
    create_level_from_vec(cells)
}

pub fn get_level_3() -> Level {
    let cells = vec![(-2,14),(3,13),(6,10),(-1,1),(7,11),(3,1),(6,11),(-4,10),(7,3),(5,12),(-1,14),(-1,0),(6,4),(-5,9),(-2,0),(-6,5),(4,14),(-4,11),(6,3),(-3,1),(5,13),(-6,9),(4,2),(-3,13),(-4,3),(-4,4),(8,10),(8,5),(-2,2),(8,4),(-5,3),(4,0),(7,5),(-3,12),(4,12),(-5,11),(5,1),(-3,2),(5,2),(-1,13),(-6,4),(-2,12),(3,0),(3,14),(-5,5),(7,9),(-6,10),(8,9),(-2,14),(3,13),(6,10),(-1,1),(7,11),(3,1),(6,11),(-4,10),(7,3),(5,12),(-1,14),(-1,0),(6,4),(-5,9),(-2,0),(-6,5),(4,14),(-4,11),(6,3),(-3,1),(5,13),(-6,9),(4,2),(-3,13),(-4,3),(-4,4),(8,10),(8,5),(-2,2),(8,4),(-5,3),(4,0),(7,5),(-3,12),(4,12),(-5,11),(5,1),(-3,2),(5,2),(-1,13),(-6,4),(-2,12),(3,0),(3,14),(-5,5),(7,9),(-6,10),(8,9),(-2,14),(3,13),(6,10),(-1,1),(7,11),(3,1),(6,11),(-4,10),(7,3),(5,12),(-1,14),(-1,0),(6,4),(-5,9),(-2,0),(-6,5),(4,14),(-4,11),(6,3),(-3,1),(5,13),(-6,9),(4,2),(-3,13),(-4,3),(-4,4),(8,10),(8,5),(-2,2),(8,4),(-5,3),(4,0),(7,5),(-3,12),(4,12),(-5,11),(5,1),(-3,2),(5,2),(-1,13),(-6,4),(-2,12),(3,0),(3,14),(-5,5),(7,9),(-6,10),(8,9),(-2,14),(3,13),(6,10),(-1,1),(7,11),(3,1),(6,11),(-4,10),(7,3),(5,12),(-1,14),(-1,0),(6,4),(-5,9),(-2,0),(-6,5),(4,14),(-4,11),(6,3),(-3,1),(5,13),(-6,9),(4,2),(-3,13),(-4,3),(-4,4),(8,10),(8,5),(-2,2),(8,4),(-5,3),(4,0),(7,5),(-3,12),(4,12),(-5,11),(5,1),(-3,2),(5,2),(-1,13),(-6,4),(-2,12),(3,0),(3,14),(-5,5),(7,9),(-6,10),(8,9),(-2,14),(3,13),(6,10),(-1,1),(7,11),(3,1),(6,11),(-4,10),(7,3),(5,12),(-1,14),(-1,0),(6,4),(-5,9),(-2,0),(-6,5),(4,14),(-4,11),(6,3),(-3,1),(5,13),(-6,9),(4,2),(-3,13),(-4,3),(-4,4),(8,10),(8,5),(-2,2),(8,4),(-5,3),(4,0),(7,5),(-3,12),(4,12),(-5,11),(5,1),(-3,2),(5,2),(-1,13),(-6,4),(-2,12),(3,0),(3,14),(-5,5),(7,9),(-6,10),(8,9),(-2,14),(3,13),(6,10),(-1,1),(7,11),(3,1),(6,11),(-4,10),(7,3),(5,12),(-1,14),(-1,0),(6,4),(-5,9),(-2,0),(-6,5),(4,14),(-4,11),(6,3),(-3,1),(5,13),(-6,9),(4,2),(-3,13),(-4,3),(-4,4),(8,10),(8,5),(-2,2),(8,4),(-5,3),(4,0),(7,5),(-3,12),(4,12),(-5,11),(5,1),(-3,2),(5,2),(-1,13),(-6,4),(-2,12),(3,0),(3,14),(-5,5),(7,9),(-6,10),(8,9)];
    create_level_from_vec(cells)
}

pub fn get_level_4() -> Level {
    let cells = vec![(-2,9),(2,4),(4,1),(-1,2),(0,9),(2,5),(7,10),(-6,10),(-1,6),(-4,4),(3,4),(3,2),(4,2),(4,8),(2,9),(-2,14),(4,9),(-2,1),(4,14),(-4,9),(7,4),(2,10),(0,5),(-1,8),(-2,12),(-5,10),(-2,0),(6,4),(-2,5),(6,10),(6,9),(-5,4),(8,10),(-2,8),(-1,10),(4,6),(4,0),(-6,4),(0,4),(-1,12),(-4,10),(-2,13),(-2,6),(4,12),(4,13),(-4,5),(6,5),(3,12),(3,8),(-1,4),(3,10),(-2,2),(0,10),(3,6),(8,4),(4,5),(-2,9),(2,4),(4,1),(-1,2),(0,9),(2,5),(7,10),(-6,10),(-1,6),(-4,4),(3,4),(3,2),(4,2),(4,8),(2,9),(-2,14),(4,9),(-2,1),(4,14),(-4,9),(7,4),(2,10),(0,5),(-1,8),(-2,12),(-5,10),(-2,0),(6,4),(-2,5),(6,10),(6,9),(-5,4),(8,10),(-2,8),(-1,10),(4,6),(4,0),(-6,4),(0,4),(-1,12),(-4,10),(-2,13),(-2,6),(4,12),(4,13),(-4,5),(6,5),(3,12),(3,8),(-1,4),(3,10),(-2,2),(0,10),(3,6),(8,4),(4,5),(-2,9),(2,4),(4,1),(-1,2),(0,9),(2,5),(7,10),(-6,10),(-1,6),(-4,4),(3,4),(3,2),(4,2),(4,8),(2,9),(-2,14),(4,9),(-2,1),(4,14),(-4,9),(7,4),(2,10),(0,5),(-1,8),(-2,12),(-5,10),(-2,0),(6,4),(-2,5),(6,10),(6,9),(-5,4),(8,10),(-2,8),(-1,10),(4,6),(4,0),(-6,4),(0,4),(-1,12),(-4,10),(-2,13),(-2,6),(4,12),(4,13),(-4,5),(6,5),(3,12),(3,8),(-1,4),(3,10),(-2,2),(0,10),(3,6),(8,4),(4,5),(-2,9),(2,4),(4,1),(-1,2),(0,9),(2,5),(7,10),(-6,10),(-1,6),(-4,4),(3,4),(3,2),(4,2),(4,8),(2,9),(-2,14),(4,9),(-2,1),(4,14),(-4,9),(7,4),(2,10),(0,5),(-1,8),(-2,12),(-5,10),(-2,0),(6,4),(-2,5),(6,10),(6,9),(-5,4),(8,10),(-2,8),(-1,10),(4,6),(4,0),(-6,4),(0,4),(-1,12),(-4,10),(-2,13),(-2,6),(4,12),(4,13),(-4,5),(6,5),(3,12),(3,8),(-1,4),(3,10),(-2,2),(0,10),(3,6),(8,4),(4,5),(-2,9),(2,4),(4,1),(-1,2),(0,9),(2,5),(7,10),(-6,10),(-1,6),(-4,4),(3,4),(3,2),(4,2),(4,8),(2,9),(-2,14),(4,9),(-2,1),(4,14),(-4,9),(7,4),(2,10),(0,5),(-1,8),(-2,12),(-5,10),(-2,0),(6,4),(-2,5),(6,10),(6,9),(-5,4),(8,10),(-2,8),(-1,10),(4,6),(4,0),(-6,4),(0,4),(-1,12),(-4,10),(-2,13),(-2,6),(4,12),(4,13),(-4,5),(6,5),(3,12),(3,8),(-1,4),(3,10),(-2,2),(0,10),(3,6),(8,4),(4,5),(-2,9),(2,4),(4,1),(-1,2),(0,9),(2,5),(7,10),(-6,10),(-1,6),(-4,4),(3,4),(3,2),(4,2),(4,8),(2,9),(-2,14),(4,9),(-2,1),(4,14),(-4,9),(7,4),(2,10),(0,5),(-1,8),(-2,12),(-5,10),(-2,0),(6,4),(-2,5),(6,10),(6,9),(-5,4),(8,10),(-2,8),(-1,10),(4,6),(4,0),(-6,4),(0,4),(-1,12),(-4,10),(-2,13),(-2,6),(4,12),(4,13),(-4,5),(6,5),(3,12),(3,8),(-1,4),(3,10),(-2,2),(0,10),(3,6),(8,4),(4,5)];
    create_level_from_vec(cells)
}

pub fn get_level_5() -> Level {
    let cells = vec![(4,3),(7,9),(6,3),(8,5),(2,4),(2,3),(8,7),(8,3),(2,6),(3,9),(5,9),(4,9),(3,3),(2,5),(8,6),(6,9),(8,4),(2,8),(8,9),(5,3),(2,7),(8,8),(7,3),(2,9),(4,3),(7,9),(6,3),(8,5),(2,4),(2,3),(8,7),(8,3),(2,6),(3,9),(5,9),(4,9),(3,3),(2,5),(8,6),(6,9),(8,4),(2,8),(8,9),(5,3),(2,7),(8,8),(7,3),(2,9),(4,3),(7,9),(6,3),(8,5),(2,4),(2,3),(8,7),(8,3),(2,6),(3,9),(5,9),(4,9),(3,3),(2,5),(8,6),(6,9),(8,4),(2,8),(8,9),(5,3),(2,7),(8,8),(7,3),(2,9),(4,3),(7,9),(6,3),(8,5),(2,4),(2,3),(8,7),(8,3),(2,6),(3,9),(5,9),(4,9),(3,3),(2,5),(8,6),(6,9),(8,4),(2,8),(8,9),(5,3),(2,7),(8,8),(7,3),(2,9),(4,3),(7,9),(6,3),(8,5),(2,4),(2,3),(8,7),(8,3),(2,6),(3,9),(5,9),(4,9),(3,3),(2,5),(8,6),(6,9),(8,4),(2,8),(8,9),(5,3),(2,7),(8,8),(7,3),(2,9),(4,3),(7,9),(6,3),(8,5),(2,4),(2,3),(8,7),(8,3),(2,6),(3,9),(5,9),(4,9),(3,3),(2,5),(8,6),(6,9),(8,4),(2,8),(8,9),(5,3),(2,7),(8,8),(7,3),(2,9),(4,3),(7,9),(6,3),(8,5),(2,4),(2,3),(8,7),(8,3),(2,6),(3,9),(5,9),(4,9),(3,3),(2,5),(8,6),(6,9),(8,4),(2,8),(8,9),(5,3),(2,7),(8,8),(7,3),(2,9)];
    create_level_from_vec(cells)
}

pub fn get_level_6() -> Level {
    let cells = vec![(150,73),(151,68),(150,79),(151,79),(150,75),(150,68),(150,77),(150,72),(150,76),(150,70),(150,78),(151,78),(150,69),(150,67),(150,74),(150,71),(151,67),(150,73),(151,68),(150,79),(151,79),(150,75),(150,68),(150,77),(150,72),(150,76),(150,70),(150,78),(151,78),(150,69),(150,67),(150,74),(150,71),(151,67),(150,73),(151,68),(150,79),(151,79),(150,75),(150,68),(150,77),(150,72),(150,76),(150,70),(150,78),(151,78),(150,69),(150,67),(150,74),(150,71),(151,67),(150,73),(151,68),(150,79),(151,79),(150,75),(150,68),(150,77),(150,72),(150,76),(150,70),(150,78),(151,78),(150,69),(150,67),(150,74),(150,71),(151,67),(150,73),(151,68),(150,79),(151,79),(150,75),(150,68),(150,77),(150,72),(150,76),(150,70),(150,78),(151,78),(150,69),(150,67),(150,74),(150,71),(151,67),(150,73),(151,68),(150,79),(151,79),(150,75),(150,68),(150,77),(150,72),(150,76),(150,70),(150,78),(151,78),(150,69),(150,67),(150,74),(150,71),(151,67)];
    create_level_from_vec(cells)
}

pub fn get_level_7() -> Level {
    let cells = vec![(4,12),(0,3),(4,-8),(11,5),(5,6),(-6,3),(7,8),(1,9),(0,1),(12,5),(3,-2),(3,12),(1,8),(-6,2),(-3,-1),(3,-8),(1,-5),(4,-2),(5,-8),(-2,-1),(7,-6),(7,9),(0,2),(12,-1),(-6,1),(5,-2),(8,2),(-4,-1),(11,-1),(-2,5),(1,-6),(14,1),(14,2),(3,6),(7,10),(5,12),(8,1),(-3,5),(-4,5),(4,6),(1,10),(8,3),(10,-1),(1,-4),(14,3),(7,-4),(7,-5),(10,5),(4,12),(0,3),(4,-8),(11,5),(5,6),(-6,3),(7,8),(1,9),(0,1),(12,5),(3,-2),(3,12),(1,8),(-6,2),(-3,-1),(3,-8),(1,-5),(4,-2),(5,-8),(-2,-1),(7,-6),(7,9),(0,2),(12,-1),(-6,1),(5,-2),(8,2),(-4,-1),(11,-1),(-2,5),(1,-6),(14,1),(14,2),(3,6),(7,10),(5,12),(8,1),(-3,5),(-4,5),(4,6),(1,10),(8,3),(10,-1),(1,-4),(14,3),(7,-4),(7,-5),(10,5),(4,12),(0,3),(4,-8),(11,5),(5,6),(-6,3),(7,8),(1,9),(0,1),(12,5),(3,-2),(3,12),(1,8),(-6,2),(-3,-1),(3,-8),(1,-5),(4,-2),(5,-8),(-2,-1),(7,-6),(7,9),(0,2),(12,-1),(-6,1),(5,-2),(8,2),(-4,-1),(11,-1),(-2,5),(1,-6),(14,1),(14,2),(3,6),(7,10),(5,12),(8,1),(-3,5),(-4,5),(4,6),(1,10),(8,3),(10,-1),(1,-4),(14,3),(7,-4),(7,-5),(10,5),(4,12),(0,3),(4,-8),(11,5),(5,6),(-6,3),(7,8),(1,9),(0,1),(12,5),(3,-2),(3,12),(1,8),(-6,2),(-3,-1),(3,-8),(1,-5),(4,-2),(5,-8),(-2,-1),(7,-6),(7,9),(0,2),(12,-1),(-6,1),(5,-2),(8,2),(-4,-1),(11,-1),(-2,5),(1,-6),(14,1),(14,2),(3,6),(7,10),(5,12),(8,1),(-3,5),(-4,5),(4,6),(1,10),(8,3),(10,-1),(1,-4),(14,3),(7,-4),(7,-5),(10,5),(4,12),(0,3),(4,-8),(11,5),(5,6),(-6,3),(7,8),(1,9),(0,1),(12,5),(3,-2),(3,12),(1,8),(-6,2),(-3,-1),(3,-8),(1,-5),(4,-2),(5,-8),(-2,-1),(7,-6),(7,9),(0,2),(12,-1),(-6,1),(5,-2),(8,2),(-4,-1),(11,-1),(-2,5),(1,-6),(14,1),(14,2),(3,6),(7,10),(5,12),(8,1),(-3,5),(-4,5),(4,6),(1,10),(8,3),(10,-1),(1,-4),(14,3),(7,-4),(7,-5),(10,5)];
    create_level_from_vec(cells)
}

pub fn get_level_8() -> Level {
    let cells = vec![(8,10),(8,4),(-1,13),(1,-1),(2,0),(7,10),(-1,14),(-2,14),(-5,10),(-6,4),(-5,4),(-6,10),(2,14),(-2,1),(-2,0),(3,1),(-6,5),(7,8),(3,13),(-1,0),(3,14),(-5,9),(-2,13),(4,14),(8,5),(0,14),(7,6),(-5,8),(4,1),(6,7),(4,13),(0,0),(8,9),(7,4),(-1,1),(4,0),(-5,6),(7,9),(1,15),(-5,5),(7,5),(3,0),(-6,9),(-4,7),(8,10),(8,4),(-1,13),(1,-1),(2,0),(7,10),(-1,14),(-2,14),(-5,10),(-6,4),(-5,4),(-6,10),(2,14),(-2,1),(-2,0),(3,1),(-6,5),(7,8),(3,13),(-1,0),(3,14),(-5,9),(-2,13),(4,14),(8,5),(0,14),(7,6),(-5,8),(4,1),(6,7),(4,13),(0,0),(8,9),(7,4),(-1,1),(4,0),(-5,6),(7,9),(1,15),(-5,5),(7,5),(3,0),(-6,9),(-4,7),(8,10),(8,4),(-1,13),(1,-1),(2,0),(7,10),(-1,14),(-2,14),(-5,10),(-6,4),(-5,4),(-6,10),(2,14),(-2,1),(-2,0),(3,1),(-6,5),(7,8),(3,13),(-1,0),(3,14),(-5,9),(-2,13),(4,14),(8,5),(0,14),(7,6),(-5,8),(4,1),(6,7),(4,13),(0,0),(8,9),(7,4),(-1,1),(4,0),(-5,6),(7,9),(1,15),(-5,5),(7,5),(3,0),(-6,9),(-4,7),(8,10),(8,4),(-1,13),(1,-1),(2,0),(7,10),(-1,14),(-2,14),(-5,10),(-6,4),(-5,4),(-6,10),(2,14),(-2,1),(-2,0),(3,1),(-6,5),(7,8),(3,13),(-1,0),(3,14),(-5,9),(-2,13),(4,14),(8,5),(0,14),(7,6),(-5,8),(4,1),(6,7),(4,13),(0,0),(8,9),(7,4),(-1,1),(4,0),(-5,6),(7,9),(1,15),(-5,5),(7,5),(3,0),(-6,9),(-4,7),(8,10),(8,4),(-1,13),(1,-1),(2,0),(7,10),(-1,14),(-2,14),(-5,10),(-6,4),(-5,4),(-6,10),(2,14),(-2,1),(-2,0),(3,1),(-6,5),(7,8),(3,13),(-1,0),(3,14),(-5,9),(-2,13),(4,14),(8,5),(0,14),(7,6),(-5,8),(4,1),(6,7),(4,13),(0,0),(8,9),(7,4),(-1,1),(4,0),(-5,6),(7,9),(1,15),(-5,5),(7,5),(3,0),(-6,9),(-4,7),(8,10),(8,4),(-1,13),(1,-1),(2,0),(7,10),(-1,14),(-2,14),(-5,10),(-6,4),(-5,4),(-6,10),(2,14),(-2,1),(-2,0),(3,1),(-6,5),(7,8),(3,13),(-1,0),(3,14),(-5,9),(-2,13),(4,14),(8,5),(0,14),(7,6),(-5,8),(4,1),(6,7),(4,13),(0,0),(8,9),(7,4),(-1,1),(4,0),(-5,6),(7,9),(1,15),(-5,5),(7,5),(3,0),(-6,9),(-4,7),(8,10),(8,4),(-1,13),(1,-1),(2,0),(7,10),(-1,14),(-2,14),(-5,10),(-6,4),(-5,4),(-6,10),(2,14),(-2,1),(-2,0),(3,1),(-6,5),(7,8),(3,13),(-1,0),(3,14),(-5,9),(-2,13),(4,14),(8,5),(0,14),(7,6),(-5,8),(4,1),(6,7),(4,13),(0,0),(8,9),(7,4),(-1,1),(4,0),(-5,6),(7,9),(1,15),(-5,5),(7,5),(3,0),(-6,9),(-4,7),(8,10),(8,4),(-1,13),(1,-1),(2,0),(7,10),(-1,14),(-2,14),(-5,10),(-6,4),(-5,4),(-6,10),(2,14),(-2,1),(-2,0),(3,1),(-6,5),(7,8),(3,13),(-1,0),(3,14),(-5,9),(-2,13),(4,14),(8,5),(0,14),(7,6),(-5,8),(4,1),(6,7),(4,13),(0,0),(8,9),(7,4),(-1,1),(4,0),(-5,6),(7,9),(1,15),(-5,5),(7,5),(3,0),(-6,9),(-4,7)];
    create_level_from_vec(cells)
}

pub fn get_level_9() -> Level {
    let cells = vec![(11,-2),(11,0),(11,4),(4,6),(8,-5),(2,9),(-3,5),(3,2),(7,9),(4,-3),(11,-1),(6,9),(15,2),(-5,2),(11,5),(11,6),(14,2),(4,11),(2,-5),(4,-7),(4,-1),(13,2),(-6,2),(-1,2),(1,-5),(1,2),(4,5),(4,2),(4,7),(0,2),(6,-5),(4,-9),(-3,-1),(4,13),(1,9),(4,-2),(-3,4),(-3,0),(4,-8),(-3,6),(5,2),(7,-5),(9,2),(0,-5),(-3,-2),(0,9),(4,1),(8,9),(8,2),(4,12),(-7,2),(7,2),(11,-2),(11,0),(11,4),(4,6),(8,-5),(2,9),(-3,5),(3,2),(7,9),(4,-3),(11,-1),(6,9),(15,2),(-5,2),(11,5),(11,6),(14,2),(4,11),(2,-5),(4,-7),(4,-1),(13,2),(-6,2),(-1,2),(1,-5),(1,2),(4,5),(4,2),(4,7),(0,2),(6,-5),(4,-9),(-3,-1),(4,13),(1,9),(4,-2),(-3,4),(-3,0),(4,-8),(-3,6),(5,2),(7,-5),(9,2),(0,-5),(-3,-2),(0,9),(4,1),(8,9),(8,2),(4,12),(-7,2),(7,2),(11,-2),(11,0),(11,4),(4,6),(8,-5),(2,9),(-3,5),(3,2),(7,9),(4,-3),(11,-1),(6,9),(15,2),(-5,2),(11,5),(11,6),(14,2),(4,11),(2,-5),(4,-7),(4,-1),(13,2),(-6,2),(-1,2),(1,-5),(1,2),(4,5),(4,2),(4,7),(0,2),(6,-5),(4,-9),(-3,-1),(4,13),(1,9),(4,-2),(-3,4),(-3,0),(4,-8),(-3,6),(5,2),(7,-5),(9,2),(0,-5),(-3,-2),(0,9),(4,1),(8,9),(8,2),(4,12),(-7,2),(7,2),(11,-2),(11,0),(11,4),(4,6),(8,-5),(2,9),(-3,5),(3,2),(7,9),(4,-3),(11,-1),(6,9),(15,2),(-5,2),(11,5),(11,6),(14,2),(4,11),(2,-5),(4,-7),(4,-1),(13,2),(-6,2),(-1,2),(1,-5),(1,2),(4,5),(4,2),(4,7),(0,2),(6,-5),(4,-9),(-3,-1),(4,13),(1,9),(4,-2),(-3,4),(-3,0),(4,-8),(-3,6),(5,2),(7,-5),(9,2),(0,-5),(-3,-2),(0,9),(4,1),(8,9),(8,2),(4,12),(-7,2),(7,2),(11,-2),(11,0),(11,4),(4,6),(8,-5),(2,9),(-3,5),(3,2),(7,9),(4,-3),(11,-1),(6,9),(15,2),(-5,2),(11,5),(11,6),(14,2),(4,11),(2,-5),(4,-7),(4,-1),(13,2),(-6,2),(-1,2),(1,-5),(1,2),(4,5),(4,2),(4,7),(0,2),(6,-5),(4,-9),(-3,-1),(4,13),(1,9),(4,-2),(-3,4),(-3,0),(4,-8),(-3,6),(5,2),(7,-5),(9,2),(0,-5),(-3,-2),(0,9),(4,1),(8,9),(8,2),(4,12),(-7,2),(7,2)];
    create_level_from_vec(cells)
}

pub fn get_level_0() -> Level {
    let cells = vec![(158,71),(160,72),(162,75),(160,74),(158,75),(162,71),(161,70),(162,70),(161,76),(159,74),(158,76),(162,76),(159,76),(161,74),(159,72),(161,72),(158,70),(159,70)];
    create_level_from_vec(cells)
}

pub fn get_level_random(max_x: i64, max_y: i64) -> Level {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(2500, 5000);
    let cells = (0..num).map(|_| (rng.gen_range(0, max_x), rng.gen_range(0, max_y))).collect();
    create_level_from_vec(cells)
}

fn create_level_from_vec(cells: Vec<(i64,i64)>) -> Level {
    let mut level = HashMap::new();
    for c in cells {
        level.insert(c, 1);
    }
    level
}