use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct CartesianCoordinate {
    pub x: f64,
    pub y: f64,
}

pub struct PolarCoordinate {
    pub r: f64,
    pub theta: f64,
}

pub struct Matrix([[f64; 2]; 2]);

pub struct EchoServer;

pub trait Coordinates {
    fn to_cartesian(self) -> CartesianCoordinate;
    fn from_cartesian(cart: CartesianCoordinate) -> Self;
}

pub trait LinerTransform: Coordinates {
    fn transform(self, matrix: &Matrix) -> Self
    where
        Self: Sized,
    {
        let mut cart = self.to_cartesian();
        let x = cart.x;
        let y = cart.y;
        let m = matrix.0;
        cart.x = m[0][0] * x + m[0][1] * y;
        cart.y = m[1][0] * x + m[1][1] * y;
        Self::from_cartesian(cart)
    }
    fn rotate(self, theta: f64) -> Self
    where
        Self: Sized,
    {
        self.transform(&Matrix([
            [theta.cos(), -theta.sin()],
            [theta.sin(), theta.cos()],
        ]))
    }

}
pub trait Init<T> {
    fn init(t: T) -> Self;
}

pub trait As<T> {
    fn cast(self) -> T;
}

pub trait Dimension {
    const DIMENSION: u32;
}

pub trait Server {
    type Response;
    type Request: FromStr;

    fn handle(&self, req: Self::Request) -> Self::Response;
    // fn handle1<S: Server>(&self, server: S, req: &str) -> S::Response;
}

impl Coordinates for CartesianCoordinate {
    fn to_cartesian(self) -> Self { self }
    fn from_cartesian(cart: Self) -> Self { cart }
}

impl Coordinates for PolarCoordinate {
    fn to_cartesian(self) -> CartesianCoordinate {
        CartesianCoordinate {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }

    fn from_cartesian(cart: CartesianCoordinate) -> Self {
        Self {
            r: (cart.x.powf(2.0) + cart.y.powf(2.0)).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}

impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCoordinate {
        CartesianCoordinate {
            x: self.0,
            y: self.1,
        }
    }

    fn from_cartesian(cart: CartesianCoordinate) -> Self {
        (cart.x, cart.y)
    }
}

impl LinerTransform for CartesianCoordinate {
    fn transform(mut self, matrix: &Matrix) -> Self {
        let x = self.x;
        let y = self.y;
        let m = matrix.0;

        self.x = m[0][0] * x + m[0][1] * y;
        self.y = m[1][0] * x + m[1][1] * y;
        self
    }

}

impl LinerTransform for PolarCoordinate {
    fn transform(self, _matrix: &Matrix) -> Self { self }
    fn rotate(mut self, theta: f64) -> Self {
        self.theta += theta;
        self
    }
}

impl<T> Init<T> for Box<T> {
    fn init(t: T) -> Self {
        Box::new(t)
    }
}

impl As<u64> for u8 {
    fn cast(self) -> u64 {
        self as u64
    }
}

impl As<u32> for u8 {
    fn cast(self) -> u32 {
        self as u32
    }
}

impl Dimension for CartesianCoordinate {
    const DIMENSION: u32 = 15;
}

impl Server for EchoServer {
    type Response = String;
    type Request = String;
    fn handle(&self, req: Self::Request) -> Self::Response {
        req
    }
    // resigned....
    // fn handle1<S: Server>(&self, server: S, req: &str) -> S::Response {
    //     let req = Self::Request::from_str(&req).unwrap();
    //     server.handle1(req)
    // }
}