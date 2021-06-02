struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Vector3 {
        return Vector3 {x, y, z};
    }
    //Add to X Y and Z axis individually
    fn add(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
    ///Add to all 3 axis of Vector
    fn add_all(&mut self, amount: f64) {
        self.x += amount;
        self.y += amount;
        self.z += amount;
    }
    ///Returns the length of the Vector
    fn length(&self) -> f64 {
        return self.x*self.x+self.y*self.y+self.z*self.z;
    }
    ///Returns the distance between the Vector3 instance and the specified other Vector3
    fn distance_to(&self, other: &Vector3) -> f64 {
        return ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0) + (self.z - other.z).powf(2.0)).sqrt();
    }
}


fn main() {
    let my_vector = Vector3::new(309.22, 232.34, 132.32213);
    let my_other_vector = Vector3::new(23.2313, 232.54, 132.2);

    println!("Die Differenz beträgt {}", my_vector.distance_to(&my_other_vector));
}