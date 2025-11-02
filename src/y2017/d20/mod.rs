use std::collections::HashMap;

use regex::Regex;

struct Particle {
    id: usize,
    p: Vec<i32>,
    v: Vec<i32>,
    a: Vec<i32>,
    p_str: String,
}

fn pos_str(p: &Vec<i32>) -> String {
    p.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn parse_particles(data: &str) -> Vec<Particle> {
    let re = Regex::new(r"<([^<>]*)>").unwrap();
    let mut particles = Vec::new();
    for (i, line) in data.lines().enumerate() {
        let parts: Vec<Vec<_>> = re
            // ChatGPT
            .captures_iter(line)
            .map(|cap| {
                cap[1]
                    .trim()
                    .split(',')
                    .map(|x| x.parse::<i32>().expect("not a number"))
                    .collect()
            })
            .collect();
        assert!(parts.len() == 3);

        // ChatGPT
        let mut it = parts.into_iter();
        let p = it.next().unwrap();
        let v = it.next().unwrap();
        let a = it.next().unwrap();
        let p_str = pos_str(&p);

        particles.push(Particle {
            id: i,
            p,
            v,
            a,
            p_str,
        });
    }
    particles
}

fn closest_particle(data: &str) -> u32 {
    let mut particles = parse_particles(data);

    for _ in 0..10000 {
        for x in &mut particles {
            for i in 0..3 {
                x.v[i] += x.a[i];
                x.p[i] += x.v[i];
            }
        }
    }

    let mut min = u32::MAX;
    let mut id = -1;
    for x in &particles {
        let dist = x.p.iter().fold(0u32, |s, c| s + c.abs() as u32);
        if dist < min {
            min = dist;
            id = x.id as i32;
        }
    }

    assert!(id >= 0);
    id as u32
}

fn count_left(data: &str) -> usize {
    let mut particles = parse_particles(data);

    for _ in 0..10000 {
        if particles.len() <= 1 {
            break;
        }

        let mut pos_map: HashMap<String, usize> = HashMap::new();
        for x in &mut particles {
            for i in 0..3 {
                x.v[i] += x.a[i];
                x.p[i] += x.v[i];
            }
            x.p_str = pos_str(&x.p);
            *pos_map.entry(x.p_str.clone()).or_insert(0) += 1;
        }

        particles.retain(|x| pos_map.get(&x.p_str).copied() == Some(1));
    }

    particles.len()
}

pub fn run(input: &str) {
    println!("{}", closest_particle(input));
    println!("{}", count_left(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            closest_particle(
                "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>"
            ),
            0
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            count_left(
                "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>"
            ),
            1
        );
    }
}
