// SPDX-License-Identifier: LGPL-3.0-or-later
/// Real needs of a real human
#[derive(Debug)]
pub struct PersonNeeds {
    pub bladder: f64,
    pub comfort: f64,
    pub environment: f64,
    pub fun: f64,
    pub hunger: f64,
    pub hygiene: f64,
    pub sleep: f64,
    pub social: f64,
}

impl Default for PersonNeeds {
    fn default() -> Self {
        Self {
            bladder: 1.0,
            comfort: 1.0,
            environment: 1.0,
            fun: 1.0,
            hunger: 1.0,
            hygiene: 1.0,
            sleep: 1.0,
            social: 1.0,
        }
    }
}

impl PersonNeeds {
    const NEED_BLADDER_DECAY_RATE: f64 = 0.01;
    const NEED_COMFORT_DECAY_RATE: f64 = 0.01;
    const NEED_FUN_DECAY_RATE: f64 = 0.01;
    const NEED_HUNGER_DECAY_RATE: f64 = 0.01;
    const NEED_HYGIENE_DECAY_RATE: f64 = 0.01;
    const NEED_SLEEP_DECAY_RATE: f64 = 0.01;
    const NEED_SOCIAL_DECAY_RATE: f64 = 0.01;

    pub fn bladder(&self) -> f64 {
        self.bladder
    }

    pub fn comfort(&self) -> f64 {
        self.comfort
    }

    pub fn environment(&self) -> f64 {
        self.environment
    }

    pub fn fun(&self) -> f64 {
        self.fun
    }

    pub fn hunger(&self) -> f64 {
        self.hunger
    }

    pub fn hygiene(&self) -> f64 {
        self.hygiene
    }

    pub fn sleep(&self) -> f64 {
        self.sleep
    }

    pub fn social(&self) -> f64 {
        self.social
    }

    pub fn set_bladder(&mut self, value: f64) {
        self.bladder = value.clamp(0.0, 1.0)
    }

    pub fn set_comfort(&mut self, value: f64) {
        self.comfort = value.clamp(0.0, 1.0)
    }

    pub fn set_environment(&mut self, value: f64) {
        self.environment = value.clamp(0.0, 1.0)
    }

    pub fn set_fun(&mut self, value: f64) {
        self.fun = value.clamp(0.0, 1.0)
    }

    pub fn set_hunger(&mut self, value: f64) {
        self.hunger = value.clamp(0.0, 1.0)
    }

    pub fn set_hygiene(&mut self, value: f64) {
        self.hygiene = value.clamp(0.0, 1.0)
    }

    pub fn set_sleep(&mut self, value: f64) {
        self.sleep = value.clamp(0.0, 1.0)
    }

    pub fn set_social(&mut self, value: f64) {
        self.social = value.clamp(0.0, 1.0)
    }

    pub fn update(&mut self, delta: f64) {
        self.bladder -= Self::NEED_BLADDER_DECAY_RATE * delta;
        self.comfort -= Self::NEED_COMFORT_DECAY_RATE * delta;
        self.fun -= Self::NEED_FUN_DECAY_RATE * delta;
        self.hunger -= Self::NEED_HUNGER_DECAY_RATE * delta;
        self.hygiene -= Self::NEED_HYGIENE_DECAY_RATE * delta;
        self.sleep -= Self::NEED_SLEEP_DECAY_RATE * delta;
        self.social -= Self::NEED_SOCIAL_DECAY_RATE * delta;

        self.bladder = self.bladder.clamp(0.0, 1.0);
        self.comfort = self.comfort.clamp(0.0, 1.0);
        self.fun = self.fun.clamp(0.0, 1.0);
        self.hunger = self.hunger.clamp(0.0, 1.0);
        self.hygiene = self.hygiene.clamp(0.0, 1.0);
        self.sleep = self.sleep.clamp(0.0, 1.0);
        self.social = self.social.clamp(0.0, 1.0);
    }
}
