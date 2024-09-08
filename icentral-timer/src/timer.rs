crate::ix!();


pub struct Timer {
    start: RefCell<Instant>,
    stop:  RefCell<Option<Instant>>,
}

impl Default for Timer {

    fn default() -> Self {

        debug!("creating timer");

        Self {
            start: RefCell::new(Instant::now()),
            stop:  RefCell::new(None),
        }
    }
}

impl Timer {

    pub fn start(&mut self)  {
        
        debug!("starting timer");

        *self.start.borrow_mut() = Instant::now();
    }
    
    pub fn stop(&mut self)  {

        debug!("stopping timer");
        
        *self.stop.borrow_mut() = Some(Instant::now());
    }
    
    pub fn interval(&mut self) -> Duration {

        if self.stop.borrow().is_none() {
            self.stop();
        }
        
        let interval = self.stop.borrow().unwrap() - *self.start.borrow();

        debug!("computed timer interval: {:?}", interval);

        interval
    }
}
