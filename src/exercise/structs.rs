#[derive(Debug)]
pub enum Type {
    MOTORCYCLE,
    SEDAN,
    CIVIC,
    BICYCLE,
    TRUCK,
}

#[derive(Debug)]
pub enum Condition {
    USED,
    NEW,
    REFURBISHED,
}

#[derive(Debug)]
pub struct Vehicle {
    pub(crate) v_type: Option<Type>,
    pub(crate) v_num_plate: Option<String>,
    pub(crate) price: Option<f32>,
    pub(crate) condition: Option<Condition>,
}

impl Vehicle {
    fn new() -> Self {
        Self {
            v_type: None,
            v_num_plate: None,
            price: None,
            condition: None,
        }
    }
}

#[derive(Debug)]
pub struct VehicleRoof {
    vehicles: Vec<Vehicle>, // No need for Option<Vec<Vehicle>>, just use Vec<Vehicle>
}

impl VehicleRoof {
    pub fn new() -> Self {
        Self {
            vehicles: Vec::new(), // Initialize an empty list of vehicles
        }
    }

    pub fn add_vehicle(&mut self, vehicle: Vehicle) {
        self.vehicles.push(vehicle);
    }
}
