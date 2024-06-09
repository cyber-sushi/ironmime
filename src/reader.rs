use input::{Event, Libinput, event::gesture::{GestureEvent, GestureEventTrait, GestureEndEvent, GestureSwipeEvent, GesturePinchEvent, GestureHoldEvent, GestureEventCoordinates, GesturePinchEventTrait}};
use std::{collections::HashMap, env, thread, time};
use crate::command::*;
use crate::parser::{Direction, Scale, Gesture, GestureType, parse_config};


#[derive(Debug, Default)]
struct Coords {
    x: f64,
    y: f64,
}

pub struct Reader {
    input: Libinput,
    coords: Coords,
    config: (HashMap<Gesture, String>, bool),
}

impl Reader {
    pub fn new(input: Libinput) -> Self {
        Self {
            input,
            coords: Coords::default(),
            config: parse_config(),
        }
    }

    fn reset_coords(&mut self) {
        self.coords = Coords::default();
    }

    pub fn start_loop(&mut self) {
        let user = env::var("USER").unwrap();
        loop {
            thread::sleep(time::Duration::from_millis(1));
            self.input.dispatch().unwrap();
            while let Some(Event::Gesture(event)) = self.input.next() {
                match event {
                    GestureEvent::Swipe(ref gesture_event) => {
                        match gesture_event {
                            GestureSwipeEvent::Begin(_) => {
                                self.reset_coords();
                            },
                            GestureSwipeEvent::Update(update_event) => {
                                self.coords.x += update_event.dx();
                                self.coords.y += update_event.dy();
                            },
                            GestureSwipeEvent::End(end_event) => {
                                if !end_event.cancelled() {
                                    let direction = if self.coords.x.abs() > self.coords.y.abs() {
                                        if self.coords.x > 0.0 {
                                            Direction::Right
                                        } else {
                                            Direction::Left
                                        }
                                    } else {
                                        if self.coords.y > 0.0 {
                                            Direction::Down
                                        } else {
                                            Direction::Up
                                        }
                                    };
                                    let gesture = Gesture {
                                        gesture_type: GestureType::Swipe(direction),
                                        fingers: end_event.finger_count(),
                                    };
                                    run_command(gesture, &self.config, &user);
                                }
                            },
                            _ => {},
                        }
                    },
                    GestureEvent::Pinch(ref gesture_event) => {
                        match gesture_event {
                            GesturePinchEvent::Begin(_) => {
                                self.reset_coords();
                            },
                            GesturePinchEvent::End(end_event) => {
                                if !end_event.cancelled() {
                                    let scale = if end_event.scale() > 1.0 {
                                        Scale::Out
                                    } else {
                                        Scale::In
                                    };
                                    let gesture = Gesture {
                                        gesture_type: GestureType::Pinch(scale),
                                        fingers: end_event.finger_count(),
                                    };
                                    run_command(gesture, &self.config, &user);
                                }
                            },
                            _ => {},
                        }
                    },
                    GestureEvent::Hold(ref gesture_event) => {
                        match gesture_event {
                            GestureHoldEvent::Begin(_) => {
                                self.reset_coords();
                            }
                            GestureHoldEvent::End(end_event) => {
                                if !end_event.cancelled() {
                                    let gesture = Gesture {
                                        gesture_type: GestureType::Hold,
                                        fingers: end_event.finger_count(),
                                    };
                                    run_command(gesture, &self.config, &user);
                                }
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            }
        }
    }
}

