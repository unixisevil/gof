use rand::{Rng, thread_rng};
use std::{fmt, thread, time, mem};

struct World(Vec<Vec<bool>>);

impl World  {
    fn new(h: usize, w:usize) -> Self {
        World(vec![vec![false;w];h])
    }

    fn height_wide(&self) ->  (usize, usize){
        (self.0.len(), self.0[0].len())
    }

    fn set(&mut self, x: usize, y: usize, val: bool){
        self.0[y][x] = val;
    }


    fn seed(&mut self) {
        let  (h, w) = self.height_wide();
        let mut rng  = thread_rng();
        for _  in 0 .. h*w /3 {
            self.set(rng.gen_range(0,w) , rng.gen_range(0, h), true);
        }
    }

    fn alive(&self, x: isize, y: isize) -> bool {
        let  (h, w) = self.height_wide();
        let  (ih, iw) = (h as isize, w as isize);
        if  x < 0 || x >= iw  || y < 0 ||  y >= ih {
            return  false
        }
        self.0[y as usize][x as usize]
    }

    fn neighbors(&self, x: isize, y: isize) ->  usize {
         (-1..= 1).flat_map(|e| vec![(e,-1),(e,0),(e,1)]).
        filter(|&(a,b)| !(a == 0 && b ==0) && self.alive(x+a,y+b)).count()
    }

    fn next(&self, x: isize, y: isize) -> bool {
         match  self.neighbors(x, y) {
             3 =>  true,
             2 if self.alive(x, y) => true,
             _ => false,
         }
    }

    fn migrate(&self, other: &mut World) {
        let  (h, w) = self.height_wide();
        for y  in  0..h {
            for x in 0..w {
                other.set(x, y, self.next(x as isize , y as isize))
            }
        }
    }

}

impl fmt::Display  for  World {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result{
        let  mut res = String::new();
        let  (h, w) = self.height_wide();

        for y  in  0..h {
            for x in 0..w {
                match self.0[y][x] {
                     true =>   res.push_str("\u{001b}[32;1m#"),
                     false =>  res.push_str("\u{001b}[30;1m#"),
                }
            }
            res.push('\n');
        }
        write!(dest, "{}", res)
    }
}

fn main() {
    let (mut wa, mut wb) = (World::new(30, 30),World::new(30, 30));
    wa.seed();
    for _ in  0..1024 {
        wa.migrate(&mut wb);
        println!("{}\u{001b}[0m", wa);
        thread::sleep(time::Duration::from_millis(5));
        mem::swap(&mut wa, &mut wb);
    }
}
