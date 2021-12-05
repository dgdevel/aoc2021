
use super::aocutil;
use regex::Regex;

#[derive(Debug)]
struct Point {
    x : i32,
    y : i32
}

#[derive(Debug)]
struct Line {
    from : Point,
    to : Point
}

trait Production {
    fn points(&self) -> Vec<Point>;
}

impl Production for Line {
    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        if self.from.x == self.to.x {
            let sign = (self.to.y - self.from.y).signum();
            for m in 0..(self.from.y - self.to.y).abs() + 1 {
                points.push(Point {
                    x : self.from.x,
                    y : self.from.y + (sign * m)
                });
            }
        } else if self.from.y == self.to.y {
            let sign = (self.to.x - self.from.x).signum();
            for m in 0..(self.from.x - self.to.x).abs() + 1 {
                points.push(Point {
                    x : self.from.x + (sign * m),
                    y : self.from.y
                });
            }
        } else if (self.from.x - self.to.x).abs() == (self.from.y - self.to.y).abs() {
            let signx = (self.to.x - self.from.x).signum();
            let signy = (self.to.y - self.from.y).signum();
            for m in 0..(self.from.y - self.to.y).abs() + 1 {
                points.push(Point {
                    x : self.from.x + (signx * m),
                    y : self.from.y + (signy * m)
                });
            }
        }
        points
    }
}

pub fn p5() -> String {
    let text_lines = aocutil::read_file_to_string_list("p5_1".to_string());
    let parser = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let mut grid : [u8;1000*1000] = [0;1000*1000]; // max values from input file
    let mut counter = 0;
    let mut counter_diag = 0;
    for text_line in text_lines {
        // println!("{}", text_line);
        for cap in parser.captures_iter(text_line.as_str()) {
            let line = Line {
                from : Point {
                    x : (&cap[1]).parse::<i32>().unwrap(),
                    y : (&cap[2]).parse::<i32>().unwrap()
                },
                to : Point {
                    x : (&cap[3]).parse::<i32>().unwrap(),
                    y : (&cap[4]).parse::<i32>().unwrap()
                }
            };
            if line.from.x == line.to.x || line.from.y == line.to.y {
                for point in line.points() {
                    // println!("{:?} {} {}", point, point.x, point.y);
                    let offset = (point.x * 1000) + point.y;
                    // println!("{}", offset);
                    grid[offset as usize] += 1;
                    if grid[offset as usize] == 2 {
                        counter += 1;
                        counter_diag += 1;
                    }
                }
            } else if (line.from.x - line.to.x).abs() == (line.from.y - line.to.y).abs() {
                for point in line.points() {
                    // println!("{:?} {} {}", point, point.x, point.y);
                    let offset = (point.x * 1000) + point.y;
                    // println!("{}", offset);
                    grid[offset as usize] += 1;
                    if grid[offset as usize] == 2 {
                        counter_diag += 1;
                    }
                }
            }
        }
    }
    format!("{:?}-{:?}", counter, counter_diag).to_string()
}

