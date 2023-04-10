// @see https://candle.dev/blog/javascript-to-rust/javascript-to-rust-day-9-structs-and-behavior/

use std::fmt::Display;

struct TrafficLight {
    color: TrafficLightColor,
  }
  
  impl TrafficLight {
    pub fn new() -> Self {
      Self {
        color: TrafficLightColor::Red,
      }
    }
  
    pub fn get_state(&self) -> &TrafficLightColor {
      &self.color
    }
  
    pub fn turn_green(&mut self) {
      self.color = TrafficLightColor::Green
    }
  }
  
  enum TrafficLightColor {
    Red,
    Yellow,
    Green,
  }


  impl std::fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "Traffic light is {}", self.color)
    }
  }


  impl Display for TrafficLightColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let color_string = match self {
        TrafficLightColor::Green => "green",
        TrafficLightColor::Red => "red",
        TrafficLightColor::Yellow => "yellow",
      };
      write!(f, "{}", color_string)
    }
  }