use errors::GeneralError;

struct Point2D {

    x: i32,
    y: i32

}

impl Point2D {


    pub fn new(x: i32, y: i32) -> Result<Point2D, errors::GeneralError> {

        if x > 0 && y > 0 {
            Ok(Point2D {x, y})
        } else {
            Err(errors::GeneralError)
        }

    }

    pub fn draw(&self) -> Result((), ) {

        

    }

}