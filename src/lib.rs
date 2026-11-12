use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new() -> Self {
        Vec2 { x: 0., y: 0. }
    }
}

impl Vec2 {
    fn norm(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div for Vec2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub struct Vertex<T> {
    pub pos: Vec2,
    pub disp: Vec2,
    pub meta: Option<T>,
}

pub struct Edge(pub usize, pub usize);

pub struct Graph<T> {
    pub edges: Vec<Edge>,
    pub vertices: Vec<Vertex<T>>,
}

pub fn fruchterman_reingold<T>(width: f32, height: f32, mut graph: Graph<T>) -> Graph<T> {
    let area = width * height;
    let k = (area / (graph.vertices.len() as f32)).sqrt();

    let attr_force = |distance| (distance * distance) / k;

    let repul_force = |distane| -(k * k) / distane;

    let n_iterations = 50;
    let mut temperature = width.max(height) / 10.;
    let cooling_factor = temperature / (n_iterations as f32);

    for _ in 0..n_iterations {
        // repulsive forces
        for i in 0..graph.vertices.len() {
            let v_pos = graph.vertices[i].pos;
            let mut disp = Vec2::new();

            for j in 0..graph.vertices.len() {
                let u = &graph.vertices[j];
                if i == j {
                    continue;
                }
                let diff = u.pos - v_pos;
                disp = disp + (diff / diff.norm()) * repul_force(diff.norm());
            }
            graph.vertices[i].disp = disp;
        }

        // attractive forces
        for Edge(a, b) in &graph.edges {
            if a == b {
                // prevent self-loops from introducing NaN
                continue;
            }
            let v = &graph.vertices[*a];
            let u = &graph.vertices[*b];
            let diff = v.pos - u.pos;

            let v = &mut graph.vertices[*a];
            v.disp = v.disp - (diff / diff.norm()) * attr_force(diff.norm());
            let u = &mut graph.vertices[*b];
            u.disp = u.disp + (diff / diff.norm()) * attr_force(diff.norm());
        }

        // update pos subject to temperature constraints
        for v in &mut graph.vertices {
            v.pos = v.pos + (v.disp / v.disp.norm()) * v.disp.norm().min(temperature);
            v.pos.x = (width / 2.).min((-width / 2.).max(v.pos.x));
            v.pos.y = (height / 2.).min((-height / 2.).max(v.pos.y));
        }

        temperature -= cooling_factor;
    }

    graph
}
